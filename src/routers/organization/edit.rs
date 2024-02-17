use crate::{
    error::RouterError, models::{Account, Organization, OrganizationName}, validate::validate, DbPool
};
use actix_web::web::{self};
use diesel::prelude::*;
use uuid::Uuid;

use super::ReqOrganization;

/// Edits the org
pub async fn edit_organization(
    path: web::Path<Uuid>,
    info: web::Json<ReqOrganization>,
    pool: web::Data<DbPool>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, username, uuid as acc_uuid};
    use crate::schema::app_organization_names::dsl::{language, name};
    use crate::schema::app_organizations::dsl::*;

    let account_uuid = path.into_inner();
    let new_org = info.into_inner();

    validate(&new_org)?;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // First find the org from id
        let account: Account = app_accounts
            .filter(acc_uuid.eq(account_uuid))
            .get_result(&mut conn)?;

        let org: Organization = Organization::belonging_to(&account).get_result(&mut conn)?;

        diesel::update(&account)
            .set(username.eq(new_org.username))
            .execute(&mut conn)?;

        diesel::update(&org)
            .set((
                profile_image.eq(new_org.profile_image),
                national_id.eq(new_org.national_id),
            ))
            .execute(&mut conn)?;

        diesel::update(OrganizationName::belonging_to(&account).filter(language.eq("default")))
            .set((name.eq(new_org.name),))
            .execute(&mut conn)?;

        Ok("Updated")
    })
    .await
    .unwrap()
}
