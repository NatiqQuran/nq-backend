use actix_web::{
    web::{self, ReqData},
    HttpRequest,
};
use diesel::{dsl::exists, prelude::*, select};

use crate::{
    error::{RouterError, RouterErrorDetail},
    models::{
        Account, NewAccount, NewEmployee, NewOrganization, NewOrganizationName, Organization,
    },
    validate::validate,
    DbPool,
};

use super::ReqOrganization;

/// Add a new Org
pub async fn add(
    pool: web::Data<DbPool>,
    new_org: web::Json<ReqOrganization>,
    data: ReqData<u32>,
    req: HttpRequest,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as account_id, username};
    use crate::schema::app_employees::dsl::app_employees;
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::app_organizations;
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users, id as user_id};

    let new_org_info = new_org.into_inner();
    let user_account_id = data.into_inner();

    validate(&new_org_info)?;

    let pool = pool.into_inner();

    let mut error_detail_builder = RouterErrorDetail::builder();

    let req_ip = req.peer_addr().unwrap();

    error_detail_builder
        .req_address(req_ip)
        .request_url_parsed(req.uri().path());

    if let Some(user_agent) = req.headers().get("User-agent") {
        error_detail_builder.user_agent(user_agent.to_str().unwrap().to_string());
    }

    let error_detail = error_detail_builder.build();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_accounts.filter(username.eq(&new_org_info.username)),
        ))
        .get_result::<bool>(&mut conn)?;

        if org_exists {
            return Err(
                RouterError::from_predefined("ORGANIZATION_NAME_NOT_AVAILABLE").log_to_db(pool, error_detail),
            );
        }

        // Create new account for org
        let new_account = NewAccount {
            username: &new_org_info.username,
            account_type: &String::from("organization"),
        }
        .insert_into(app_accounts)
        .get_result::<Account>(&mut conn)?;

        let user: i32 = app_users
            .filter(user_acc_id.eq(user_account_id as i32))
            .select(user_id)
            .get_result(&mut conn)?;

        let new_organization = NewOrganization {
            creator_user_id: user,
            account_id: new_account.id,
            owner_account_id: user_account_id as i32,
            profile_image: new_org_info.profile_image,
            established_date: new_org_info.established_date,
            national_id: new_org_info.national_id,
        }
        .insert_into(app_organizations)
        .get_result::<Organization>(&mut conn)?;

        // Now add the creator user as employee to the organization
        let user_account: i32 = app_accounts
            .filter(account_id.eq(user_account_id as i32))
            .select(account_id)
            .get_result(&mut conn)?;

        NewEmployee {
            creator_user_id: user,
            employee_account_id: user_account,
            org_account_id: new_organization.account_id,
        }
        .insert_into(app_employees)
        .execute(&mut conn)?;

        // Add new name to the org
        NewOrganizationName {
            creator_user_id: user,
            account_id: new_account.id,
            language: "default".to_string(),
            name: new_org_info.primary_name,
        }
        .insert_into(app_organization_names)
        .execute(&mut conn)?;

        Ok("Created")
    })
    .await
    .unwrap()
}
