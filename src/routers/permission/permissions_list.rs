use std::collections::BTreeMap;

use crate::{
    error::RouterError,
    models::{Permission, PermissionCondition},
    routers::{
        multip,
        permission::{PermissionAccount, PermissionWithConditions, SimplePermission},
    },
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the list of Permissions
///
/// with related Conditions
pub async fn get_list_of_permissions(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<PermissionWithConditions>>, RouterError> {
    use crate::schema::app_accounts::dsl::{
        app_accounts, username as acc_username, uuid as account_uuid,
    };
    use crate::schema::app_permission_conditions::dsl::app_permission_conditions;
    use crate::schema::app_permissions::dsl::app_permissions;
    use crate::schema::app_user_names::dsl::{
        app_user_names, first_name as f_name, last_name as l_name,
    };

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // TODO: fix None Condition
        let permissions_with_conditions: Vec<(
            (
                Option<String>,
                Option<String>,
                Option<String>,
                Option<Uuid>,
                Permission,
            ),
            Option<PermissionCondition>,
        )> = app_permissions
            .left_join(app_permission_conditions)
            .left_join(app_accounts.left_join(app_user_names))
            .select((
                acc_username.nullable(),
                f_name.nullable(),
                l_name.nullable(),
                account_uuid.nullable(),
                Permission::as_select(),
                // This is for situation that there is no
                // condition related to this Permission
                Option::<PermissionCondition>::as_select(),
            ))
            .load(&mut conn)?
            .into_iter()
            .map(|(un, fname, lname, u, p, pc)| ((un, fname, lname, u, p), pc))
            .collect();

        let permissions_with_conditions_map: BTreeMap<
            SimplePermission,
            Vec<Option<PermissionCondition>>,
        > = multip(permissions_with_conditions, |(un, fname, lname, u, p)| {
            SimplePermission {
                id: p.id,
                uuid: p.uuid,
                account: PermissionAccount {
                    uuid: u,
                    username: un,
                    first_name: fname,
                    last_name: lname,
                },
                object: p.object,
                action: p.action,
            }
        });

        let result: Vec<PermissionWithConditions> = permissions_with_conditions_map
            .into_iter()
            .map(|(simple_permission, conditions)| PermissionWithConditions {
                conditions: match conditions.first() {
                    Some(_) => conditions
                        .into_iter()
                        .filter(|c| c.is_some())
                        .flatten()
                        .collect(),
                    None => vec![],
                },
                permission: simple_permission,
            })
            .collect();
        Ok(web::Json(result))
    })
    .await
    .unwrap()
}
