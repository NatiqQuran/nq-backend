use crate::{
    error::RouterError,
    models::{NewOrganizationName, OrganizationName},
    validate::validate,
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct NewName {
    name: String,

    #[validate(length(equal = 2))]
    language: String,
}

pub async fn add_name<'a>(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
    new_name_req: web::Json<NewName>,
    data: web::ReqData<u32>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, id as account_id, uuid as uuid_from_account,
    };
    use crate::schema::app_organization_names::dsl::*;
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users, id as user_id};

    let new_name = new_name_req.into_inner();
    let org_uuid = path.into_inner();
    let data = data.into_inner();

    validate(&new_name)?;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let account: i32 = app_accounts
            .filter(uuid_from_account.eq(org_uuid))
            .select(account_id)
            .get_result(&mut conn)?;

        let user: i32 = app_users
            .filter(user_acc_id.eq(data as i32))
            .select(user_id)
            .get_result(&mut conn)?;

        NewOrganizationName {
            creator_user_id: user,
            account_id: account,
            name: new_name.name,
            language: new_name.language,
        }
        .insert_into(app_organization_names)
        .get_result::<OrganizationName>(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap()
}

/// Returns the list of org names
pub async fn names(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<web::Json<Vec<OrganizationName>>, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, uuid as uuid_from_account};
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::app_organizations;

    let uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        let names = app_organizations
            .inner_join(app_accounts.inner_join(app_organization_names))
            .filter(uuid_from_account.eq(uuid))
            .select(OrganizationName::as_select())
            .load::<OrganizationName>(&mut conn)?;

        Ok(web::Json(names))
    })
    .await
    .unwrap()
}

#[derive(Deserialize)]
pub struct EditableName {
    /// New name
    name: String,
    // We dont grant user to update the existing
    // names language. the language property of the name is
    // immutable
}

/// Edits the name
pub async fn edit_name<'a>(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
    edit_name_req: web::Json<EditableName>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_organization_names::dsl::{
        app_organization_names, name as name_name, uuid,
    };

    let name_uuid = path.into_inner();
    let new_name = edit_name_req.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::update(app_organization_names.filter(uuid.eq(name_uuid)))
            .set((name_name.eq(new_name.name),))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}

/// Deletes the name as given uuid
pub async fn delete_name<'a>(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_organization_names::dsl::{app_organization_names, uuid};

    let name_uuid = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::delete(app_organization_names.filter(uuid.eq(name_uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap()
}
