use std::str::FromStr;

use crate::{
    error::RouterError,
    models::{Account, Organization, OrganizationName},
    DbPool,
};
use actix_web::web::{self};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OrgInfoUpdatebleFileds {
    username: String,
    name: String,
    profile_image: Option<String>,
    national_id: String,
}

/// Edits the org
pub async fn edit_organization(
    path: web::Path<String>,
    info: web::Json<OrgInfoUpdatebleFileds>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, username, uuid as acc_uuid};
    use crate::schema::app_organization_names::dsl::{language, name};
    use crate::schema::app_organizations::dsl::*;

    let account_uuid = path.into_inner();
    // TODO: check this user id
    //let user_id = user_id.into_inner();
    let new_org = info.into_inner();

    let update_result: Result<&'static str, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let account_uuid = Uuid::from_str(&account_uuid)?;

        // First find the org from id
        let account = app_accounts
            .filter(acc_uuid.eq(account_uuid))
            .load::<Account>(&mut conn)?;

        let org =
            Organization::belonging_to(account.get(0).unwrap()).load::<Organization>(&mut conn)?;

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let Some(org) = org.get(0) else {
            return Err(RouterError::NotFound("Organization not found".to_string()));
        };

        diesel::update(account)
            .set(username.eq(new_org.username))
            .execute(&mut conn)?;

        diesel::update(&org)
            .set((
                profile_image.eq(new_org.profile_image),
                national_id.eq(new_org.national_id),
            ))
            .execute(&mut conn)?;

        diesel::update(OrganizationName::belonging_to(account).filter(language.eq("default")))
            .set((name.eq(new_org.name),))
            .execute(&mut conn)?;

        Ok("Updated")
    })
    .await
    .unwrap();

    update_result
}
