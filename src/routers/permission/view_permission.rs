use crate::{
    error::RouterError,
    models::{Permission, PermissionCondition},
    routers::permission::{PermissionWithConditions, SimplePermission},
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the list of Permissions
///
/// with related Conditions
pub async fn get_permission(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<PermissionWithConditions>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, username as acc_username, uuid as account_uuid,
    };
    use crate::schema::app_permission_conditions::dsl::app_permission_conditions;
    use crate::schema::app_permissions::dsl::{app_permissions, uuid as uuid_from_permissions};
    use crate::schema::app_user_names::dsl::{
        app_user_names, first_name as f_name, last_name as l_name,
    };

    let requested_permission_uuid = path.into_inner();

    let permission: Result<PermissionWithConditions, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let (account_username, first_name, last_name, a_uuid, permission): (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<Uuid>,
            Permission,
        ) = app_permissions
            .filter(uuid_from_permissions.eq(requested_permission_uuid))
            .left_join(app_permission_conditions)
            .left_join(app_accounts.left_join(app_user_names))
            .select((
                acc_username.nullable(),
                f_name.nullable(),
                l_name.nullable(),
                account_uuid.nullable(),
                Permission::as_select(),
            ))
            .get_result(&mut conn)?;

        let conditions: Vec<PermissionCondition> =
            PermissionCondition::belonging_to(&permission).load(&mut conn)?;

        Ok(PermissionWithConditions {
            permission: SimplePermission {
                id: permission.id,
                uuid: permission.uuid,
                account: crate::routers::permission::PermissionAccount {
                    uuid: a_uuid,
                    username: account_username,
                    first_name,
                    last_name
                },
                object: permission.object,
                action: permission.action,
            },
            conditions,
        })
    })
    .await
    .unwrap();

    Ok(web::Json(permission?))
}
