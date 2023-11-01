use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::error::RouterError;
use crate::models::Organization;
use crate::DbPool;

#[derive(Serialize, Clone)]
pub struct ViewableOrganizationData {
    pub username: String,
    pub name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
}

/// View Org data
/// path -> account_uuid related to the org
pub async fn view(
    path: web::Path<Uuid>,
    conn: web::Data<DbPool>,
) -> Result<web::Json<ViewableOrganizationData>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, id as account_id, username as account_username, uuid as acc_uuid,
    };
    use crate::schema::app_organization_names::dsl::{app_organization_names, language, name};
    use crate::schema::app_organizations::dsl::{account_id as org_account_id, app_organizations};

    let account_uuid = path.into_inner();

    web::block(move || {
        let mut conn = conn.get().unwrap();

        // Find the account
        let (account, username): (i32, String) = app_accounts
            .filter(acc_uuid.eq(account_uuid))
            .select((account_id, account_username))
            .get_result(&mut conn)?;

        let org: Organization = app_organizations
            .filter(org_account_id.eq(account))
            .get_result(&mut conn)?;

        let org_name: String = app_organization_names
            .filter(language.eq("default"))
            .select(name)
            .get_result(&mut conn)?;

        let org = ViewableOrganizationData {
            username,
            name: org_name,
            profile_image: org.profile_image,
            established_date: org.established_date,
            national_id: org.national_id,
        };

        Ok(web::Json(org))
    })
    .await
    .unwrap()
}
