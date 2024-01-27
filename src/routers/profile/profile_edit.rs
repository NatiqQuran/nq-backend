use actix_web::web;
use diesel::prelude::*;

use crate::{
    error::RouterError,
    models::{Account, User, UserName},
    routers::user::EditableUser,
    DbPool,
};

pub async fn profile_edit(
    user_id: web::ReqData<u32>,
    pool: web::Data<DbPool>,
    new_user: web::Json<EditableUser>,
) -> Result<&'static str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as account_id, username};
    use crate::schema::app_user_names::dsl::{first_name, last_name, primary_name};
    use crate::schema::app_users::dsl::*;

    let user_id = user_id.into_inner();
    let new_user = new_user.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // First find the account from id
        let account: Account = app_accounts
            .filter(account_id.eq(user_id as i32))
            .get_result(&mut conn)?;

        let user: User = User::belonging_to(&account).get_result(&mut conn)?;

        // Now update the account username
        diesel::update(&account)
            .set(username.eq(new_user.username))
            .execute(&mut conn)?;

        // And update the other data
        diesel::update(&user)
            .set((
                birthday.eq(new_user.birthday),
                profile_image.eq(new_user.profile_image),
            ))
            .execute(&mut conn)?;

        // Also edit the primary name

        // First We get the user_names of the account
        // We assume that user has at least primary name
        let name = UserName::belonging_to(&account)
            .filter(primary_name.eq(true))
            .first::<UserName>(&mut conn)?;

        // Now we update it
        diesel::update(&name)
            .set((
                first_name.eq(new_user.first_name),
                last_name.eq(new_user.last_name),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap()
}
