use actix_web::web::{self, ReqData};
use diesel::{dsl::exists, prelude::*, select};

use crate::{
    error::RouterError,
    models::{
        Account, NewAccount, NewEmployee, NewOrganization, NewOrganizationName, Organization,
    },
    validate::validate,
    DbPool,
};

use super::new_organization_info::NewOrgInfo;

/// Add a new Org
pub async fn add(
    conn: web::Data<DbPool>,
    new_org: web::Json<NewOrgInfo>,
    data: ReqData<u32>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as account_id, username};
    use crate::schema::app_employees::dsl::app_employees;
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::app_organizations;
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users, id as user_id};

    let new_org_info = new_org.into_inner();
    let user_account_id = data.into_inner();

    validate(&new_org_info)?;

    web::block(move || {
        let mut conn = conn.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_accounts.filter(username.eq(&new_org_info.username)),
        ))
        .get_result::<bool>(&mut conn)?;

        if org_exists {
            return Err(RouterError::NotAvailable(
                "organization username".to_string(),
            ));
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
            name: new_org_info.name,
        }
        .insert_into(app_organization_names)
        .execute(&mut conn)?;

        Ok("Created")
    })
    .await
    .unwrap()
}
