use actix_web::web::{self, ReqData};
use diesel::{dsl::exists, prelude::*, select};

use crate::{
    error::RouterError,
    models::{Account, NewAccount, NewEmployee, NewOrganization, Organization},
    validate::validate,
    DbPool,
};

use super::new_organization_info::NewOrgInfo;

/// Add a new Org
pub async fn add(
    conn: web::Data<DbPool>,
    new_org: web::Json<NewOrgInfo>,
    data: ReqData<u32>,
) -> Result<String, RouterError> {
    use crate::schema::app_accounts::dsl::*;
    use crate::schema::app_employees::dsl::app_employees;
    use crate::schema::app_organizations::dsl::app_organizations;

    let new_org_info = new_org.into_inner();
    let user_account_id = data.into_inner();

    validate(&new_org_info)?;

    let add_status: Result<String, RouterError> = web::block(move || {
        let mut conn = conn.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_accounts.filter(username.eq(&new_org_info.username)),
        ))
        .get_result::<bool>(&mut conn)
        .unwrap();

        if org_exists {
            return Err(RouterError::NotAvailable(
                "organization username".to_string(),
            ));
        }

        // Create new account for org
        let new_account: Account = NewAccount {
            username: &new_org_info.username,
            account_type: &String::from("organization"),
        }
        .insert_into(app_accounts)
        .get_result(&mut conn)
        .unwrap();

        let Ok(new_organization) = NewOrganization {
            account_id: new_account.id,
            name: new_org_info.name,
            profile_image: new_org_info.profile_image,
            established_date: new_org_info.established_date,
            national_id: new_org_info.national_id,
        }
        .insert_into(app_organizations)
        .get_result::<Organization>(&mut conn) else {
            return Err(RouterError::InternalError);
        };

        // Now add the creator user as employee to the organization
        let Ok(user_account)= app_accounts
            .filter(id.eq(user_account_id as i32))
            .get_result::<Account>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        let Ok(_new_employee) = NewEmployee {
            employee_account_id: user_account.id,
            org_account_id: new_organization.account_id,
        }.insert_into(app_employees).execute(&mut conn) else {
            return Err(RouterError::InternalError);
        };

        Ok("Created".to_string())
    })
    .await
    .unwrap();

    add_status
}
