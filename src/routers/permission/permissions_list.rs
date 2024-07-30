use std::collections::BTreeMap;

use crate::{
    error::RouterError,
    models::{Permission, PermissionCondition},
    routers::{
        multip,
        permission::{PermissionWithConditions, SimplePermission},
    },
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;

/// Returns the list of Permissions
///
/// with related Conditions
pub async fn get_list_of_permissions(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<PermissionWithConditions>>, RouterError> {
    use crate::schema::app_permission_conditions::dsl::app_permission_conditions;
    use crate::schema::app_permissions::dsl::app_permissions;

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // TODO: fix None Condition
        let permissions_with_conditions: Vec<(Permission, Option<PermissionCondition>)> =
            app_permissions
                .left_join(app_permission_conditions)
                .select((
                    Permission::as_select(),
                    // This is for situation that there is no
                    // condition related to this Permission
                    Option::<PermissionCondition>::as_select(),
                ))
                .load(&mut conn)?;

        let permissions_with_conditions_map: BTreeMap<
            SimplePermission,
            Vec<Option<PermissionCondition>>,
        > = multip(permissions_with_conditions, |p: Permission| {
            SimplePermission {
                id: p.id,
                uuid: p.uuid,
                subject: p.subject,
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
