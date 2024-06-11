use std::sync::Arc;

use crate::error::RouterError;
use crate::models::{Organization, User};
use crate::select_model::SelectModel;
use crate::DbPool;
use actix_web::{web, ResponseError};
use async_trait::async_trait;
use auth_z::{CheckPermission, GetModel, ModelPermission, ParsedPath};
use diesel::prelude::*;

const ANY_FILTER: char = '*';

#[derive(Debug)]
/// Request Action
enum Action {
    /// Create (POST) request to a controller
    Create,

    /// Edit (POST) request with id to a controller
    Edit,

    /// Delete (DELETE) request with id to a controller
    Delete,

    /// View (GET) request to a controller, id is not required
    View,
}

impl Action {
    fn from_auth_z<'a>(path: &ParsedPath, method: &'a str) -> Self {
        // Checks the id of path and request method
        match (path.id.clone(), method) {
            (Some(_), "GET") => Self::View,
            (None, "POST") => Self::Create,
            (Some(_), "POST") => Self::Edit,
            (Some(_), "DELETE") => Self::Delete,
            (None, _) => Self::View,
            (Some(_), _) => Self::View,
        }
    }
}

impl Into<&str> for Action {
    fn into(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Edit => "edit",
            Self::Delete => "delete",
            Self::View => "view",
        }
    }
}

#[derive(Debug, Clone)]
/// Actual Context of AuthZ
pub struct AuthZController {
    db_pool: DbPool,
}

impl AuthZController {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CheckPermission for AuthZController {
    async fn check(
        &self,
        subject: Option<u32>,
        path: ParsedPath,
        method: String,
    ) -> Result<(), Box<dyn ResponseError>> {
        use crate::schema::app_permission_conditions::dsl::{
            app_permission_conditions, name, value,
        };
        use crate::schema::app_permissions::dsl::{
            action as permission_action, app_permissions, id as permission_id,
            object as permission_object, subject as permission_subject,
        };

        let permission_denied_error =
            Box::new(RouterError::from_predefined("AUTHZ_PERMISSION_DENIED"));

        // these will be moved to the web::block closure
        let subject_copy = subject.clone();
        let path_copy = path.clone();

        let mut conn = self.db_pool.get().unwrap();
        let select_result: Result<(Vec<i32>, Vec<(String, String)>), RouterError> =
            web::block(move || {
                // Default subject query
                let subject_query = match subject_copy {
                    Some(subject) => vec![subject.to_string(), ANY_FILTER.to_string()],
                    None => vec![ANY_FILTER.to_string()],
                };

                // Found the requested Action
                let calculated_action = Action::from_auth_z(&path_copy, method.as_str());

                // Check the permissions and get the conditions
                let permissions_filter = app_permissions
                    .filter(permission_subject.eq_any(subject_query))
                    .filter(permission_object.eq(path_copy.controller.unwrap().clone()))
                    .filter(permission_action.eq::<&str>(calculated_action.into()));

                let permissions = permissions_filter
                    .clone()
                    .select(permission_id)
                    .load(&mut conn)?;

                let conditions = permissions_filter
                    .inner_join(app_permission_conditions)
                    .select((name, value))
                    .load(&mut conn)?;

                Ok((permissions, conditions))
            })
            .await
            .unwrap();

        let Ok(select_result) = select_result else {
            permission_denied_error.log_to_db(Arc::new(self.db_pool.clone()));
            return Err(permission_denied_error);
        };

        if select_result.0.is_empty() {
            permission_denied_error.log_to_db(Arc::new(self.db_pool.clone()));
            return Err(permission_denied_error);
        }

        // No need to Checking the conditions
        // there is no condition
        if select_result.1.is_empty() {
            return Ok(());
        }

        // *Now Check the conditions*

        // First get the required Resource as Model
        let model = self
            .get_model(
                &path.controller.unwrap().clone(),
                path.id.unwrap().clone().parse().unwrap(),
            )
            .await;

        // We Got the model now we check every condition
        for (cond_name, cond_value) in select_result.1 {
            let model_attr: Option<ModelAttrib> = match ModelAttrib::try_from(cond_name.as_str()) {
                Ok(v) => Some(v),

                Err(err) => {
                    err.log_to_db(Arc::new(self.db_pool.clone()));

                    None
                }
            };

            let Some(model_attr) = model_attr else {
                permission_denied_error.log_to_db(Arc::new(self.db_pool.clone()));
                return Err(permission_denied_error);
            };

            let attr = model.get_attr(model_attr.clone()).await;

            let inner_subject = match subject {
                Some(id) => Some(id.to_string()),
                None => None,
            };

            let result = ModelAttribResult::from(model_attr).validate(
                attr,
                inner_subject.as_deref(),
                &cond_value,
            );

            if result {
                return Ok(());
            }
        }

        permission_denied_error.log_to_db(Arc::new(self.db_pool.clone()));
        return Err(permission_denied_error);
    }
}

#[async_trait]
impl GetModel<ModelAttrib, i32> for AuthZController {
    async fn get_model(
        &self,
        resource_name: &str,
        resource_id: u32,
    ) -> Box<dyn ModelPermission<ModelAttrib, i32>> {
        //let mut conn = self.db_pool.get().unwrap();
        let resource_id = resource_id as i32;

        // Resource must have been impl the Model permission trait
        let model: Box<dyn ModelPermission<ModelAttrib, i32>> = match resource_name {
            "user" => Box::new(User::from_id(self.db_pool.clone(), resource_id).await),

            "organization" => {
                Box::new(Organization::from_id(self.db_pool.clone(), resource_id).await)
            }

            _ => todo!(),
        };

        model
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionValueType {
    Boolean,
}

impl TryFrom<&str> for ConditionValueType {
    type Error = RouterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "true" | "false" => Ok(Self::Boolean),

            _ => Err(RouterError::from_predefined(
                "AUTHZ_CONDITION_VALUE_NOT_DEFINED",
            )),
        }
    }
}

pub trait Condition<'a> {
    /// Validates the condition based on subject and value
    fn validate(
        &self,
        attribute: Option<i32>,
        subject: Option<&'a str>,
        condition_value: &'a str,
    ) -> bool
    where
        Self: Sized;

    /// Returns the value type of the condition
    fn get_value_type(&self) -> ConditionValueType
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct Owner;

impl<'a> Condition<'a> for Owner {
    // Validates the Owner Condition
    fn validate(
        &self,
        attr: Option<i32>,
        subject: Option<&'a str>,
        condition_value: &'a str,
    ) -> bool {
        let Some(subject) = subject else {
            return false;
        };

        if condition_value == "true" {
            matches!(attr, Some(_)) && subject == attr.unwrap().to_string()
        } else if condition_value == "false" {
            matches!(attr, None) || subject != attr.unwrap().to_string()
        } else {
            true
        }
    }

    fn get_value_type(&self) -> ConditionValueType {
        ConditionValueType::Boolean
    }
}

#[derive(Debug, Clone)]
pub struct Login;

impl<'a> Condition<'a> for Login {
    // Validates the Owner Condition
    fn validate(
        &self,
        _attr: Option<i32>,
        subject: Option<&'a str>,
        _condition_value: &'a str,
    ) -> bool {
        matches!(subject, Some(_))
    }

    fn get_value_type(&self) -> ConditionValueType {
        ConditionValueType::Boolean
    }
}

#[derive(Debug, Clone)]
pub enum ModelAttribResult {
    /// Owner Condition Result
    Owner(Owner),

    /// Login Condition Result
    Login(Login),
}

impl<'a> Condition<'a> for ModelAttribResult {
    fn validate(
        &self,
        attribute: Option<i32>,
        subject: Option<&'a str>,
        condition_value: &'a str,
    ) -> bool {
        match self {
            Self::Owner(owner) => owner.validate(attribute, subject, condition_value),
            Self::Login(login) => login.validate(attribute, subject, condition_value),
        }
    }

    fn get_value_type(&self) -> ConditionValueType {
        match self {
            Self::Owner(owner) => owner.get_value_type(),
            Self::Login(login) => login.get_value_type(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelAttrib {
    Owner,
    Login,
}

impl From<ModelAttrib> for ModelAttribResult {
    // From ModelAttrib return the Result Enum, so we can
    // validate the Condition
    fn from(value: ModelAttrib) -> Self {
        match value {
            ModelAttrib::Owner => ModelAttribResult::Owner(Owner {}),
            ModelAttrib::Login => ModelAttribResult::Login(Login {}),
        }
    }
}

// Maybe we can use TryFrom
impl TryFrom<&str> for ModelAttrib {
    type Error = RouterError;

    // Returns ModelAttrib from &str (string)
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "isOwner" => Ok(Self::Owner),
            "isLoggedIn" => Ok(Self::Login),

            _ => Err(RouterError::from_predefined("MODEL_ATTRIBUTE_NOT_DEFINED")),
        }
    }
}

#[async_trait]
impl ModelPermission<ModelAttrib, i32> for User {
    async fn get_attr(&self, name: ModelAttrib) -> Option<i32> {
        match name {
            ModelAttrib::Owner => Some(self.account_id),
            ModelAttrib::Login => None,
        }
    }
}

#[async_trait]
impl ModelPermission<ModelAttrib, i32> for Organization {
    async fn get_attr(&self, name: ModelAttrib) -> Option<i32> {
        match name {
            ModelAttrib::Owner => Some(self.owner_account_id),
            ModelAttrib::Login => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Condition, Login, ModelAttrib, Owner};

    #[test]
    fn test_login_condition() {
        let login = Login {};

        // This should return true
        assert_eq!(login.validate(None, Some("user"), ""), true);

        // This should return false
        assert_eq!(login.validate(None, None, ""), false);
    }

    #[test]
    fn test_owner_condition() {
        let owner = Owner {};

        // This should return true
        assert_eq!(owner.validate(None, Some("user"), ""), true);

        // This should return false
        assert_eq!(owner.validate(None, None, ""), false);

        assert_eq!(owner.validate(Some(1), Some("1"), "true"), true);
    }

    #[test]
    fn test_model_attrib() {
        assert_eq!(
            ModelAttrib::try_from("isOwner").unwrap(),
            ModelAttrib::Owner
        );
        assert_eq!(
            ModelAttrib::try_from("isLoggedIn").unwrap(),
            ModelAttrib::Login
        );
    }
}
