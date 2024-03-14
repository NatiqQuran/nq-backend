use crate::schema::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Selectable, Identifiable, Queryable, Debug, Serialize)]
#[diesel(table_name = app_accounts)]
pub struct Account {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub account_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = app_accounts)]
pub struct NewAccount<'a> {
    pub username: &'a String,
    pub account_type: &'a String,
}

#[derive(Clone, Identifiable, Queryable, Debug, Serialize, Associations)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_user_names)]
pub struct UserName {
    pub id: i32,
    pub account_id: i32,
    pub creator_user_id: i32,
    pub primary: bool,
    pub first_name: String,
    pub last_name: String,
    pub language: String,
}

#[derive(Insertable)]
#[diesel(table_name = app_user_names)]
pub struct NewUserNames {
    pub account_id: i32,
    pub primary_name: bool,
    pub creator_user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub language: Option<String>,
}

#[derive(Identifiable, Queryable, Debug)]
#[diesel(table_name = app_verify_codes)]
pub struct VerifyCode {
    pub id: i32,
    pub code: i32,
    pub email: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_verify_codes)]
pub struct NewVerifyCode<'a> {
    pub status: &'a String,
    pub code: &'a i32,
    pub email: &'a String,
}

#[derive(Associations, Identifiable, Queryable, Debug, Clone, Serialize, Selectable)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_users)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: i32,

    #[serde(skip_serializing)]
    pub account_id: i32,

    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
    pub language: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
    pub language: String,
}

#[derive(Insertable)]
#[diesel(table_name = app_users)]
pub struct NewUser {
    pub account_id: i32,
    pub language: Option<String>,
}

// TODO: use belongs to
#[derive(Identifiable, Queryable, Debug, Clone)]
#[diesel(table_name = app_tokens)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token_hash: String,
    pub terminated: bool,
    pub teminated_by_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = app_tokens)]
pub struct NewToken<'a> {
    pub account_id: i32,
    pub token_hash: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_emails)]
pub struct Email {
    pub id: i32,
    pub account_id: i32,
    pub creator_user_id: i32,
    pub email: String,
    pub verified: bool,
    pub primary: bool,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_emails)]
pub struct NewEmail<'a> {
    pub account_id: i32,
    pub creator_user_id: i32,
    pub email: &'a String,
    pub verified: bool,
    pub primary: bool,
    pub deleted: bool,
}

#[derive(Selectable, Identifiable, Associations, Queryable, PartialEq, Debug, Serialize, Clone)]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(table_name = app_organizations)]
pub struct Organization {
    pub id: i32,
    pub account_id: i32,
    pub owner_account_id: i32,
    pub creator_user_id: i32,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Selectable, Clone, Identifiable, Queryable, Debug, Serialize, Associations)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = app_organization_names)]
pub struct OrganizationName {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,
    pub creator_user_id: i32,

    #[serde(skip_serializing)]
    pub account_id: i32,
    pub name: String,
    pub language: String,
}

#[derive(Insertable)]
#[diesel(table_name = app_organization_names)]
pub struct NewOrganizationName {
    pub creator_user_id: i32,
    pub account_id: i32,
    pub name: String,
    pub language: String,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = app_organizations)]
pub struct NewOrganization {
    pub creator_user_id: i32,
    pub account_id: i32,
    pub owner_account_id: i32,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
}

#[derive(Queryable, Deserialize, Validate)]
#[diesel(table_name = app_employees)]
pub struct Employee {
    pub id: i32,
    pub org_account_id: i32,
    pub creator_user_id: i32,
    pub employee_account_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = app_employees)]
pub struct NewEmployee {
    pub org_account_id: i32,
    pub creator_user_id: i32,
    pub employee_account_id: i32,
}

#[derive(
    Selectable,
    Insertable,
    Deserialize,
    Validate,
    Queryable,
    Identifiable,
    Serialize,
    Associations,
    Clone,
    Debug,
)]
#[diesel(belongs_to(QuranSurah, foreign_key = surah_id))]
#[diesel(table_name = quran_ayahs)]
pub struct QuranAyah {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,
    pub creator_user_id: i32,

    #[serde(skip_serializing)]
    pub surah_id: i32,

    pub ayah_number: i32,
    pub sajdeh: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = quran_ayahs)]
pub struct NewQuranAyah {
    pub creator_user_id: i32,
    pub surah_id: i32,
    pub ayah_number: i32,
    pub sajdeh: Option<String>,
}

#[derive(Clone, Selectable, Identifiable, Associations, Queryable, PartialEq, Debug, Serialize)]
#[diesel(belongs_to(QuranAyah, foreign_key = ayah_id))]
#[diesel(table_name = quran_words)]
pub struct QuranWord {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,
    pub creator_user_id: i32,

    #[serde(skip_serializing)]
    pub ayah_id: i32,

    pub word: String,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = quran_words)]
pub struct NewQuranWord<'a> {
    pub creator_user_id: i32,
    pub ayah_id: i32,
    pub word: &'a str,
}

#[derive(Deserialize, Serialize, Clone, Validate, Identifiable, Queryable, Selectable, Debug)]
#[diesel(belongs_to(QuranMushaf, foreign_key = mushaf_id))]
#[diesel(table_name = quran_surahs)]
pub struct QuranSurah {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,

    pub creator_user_id: i32,

    pub name: String,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub mushaf_id: i32,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = quran_surahs)]
pub struct NewQuranSurah {
    pub creator_user_id: i32,
    pub name: String,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub mushaf_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Validate, Identifiable, Queryable, Selectable, Debug)]
#[diesel(table_name = mushafs)]
pub struct QuranMushaf {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,
    pub creator_user_id: i32,

    pub short_name: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,

    pub bismillah_text: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = mushafs)]
pub struct NewQuranMushaf<'a> {
    pub creator_user_id: i32,
    pub short_name: Option<&'a str>,
    pub name: Option<&'a str>,
    pub source: Option<&'a str>,
    pub bismillah_text: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Validate, Identifiable, Queryable, Debug, Selectable)]
#[diesel(table_name = app_permissions)]
pub struct Permission {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,

    pub creator_user_id: i32,

    pub subject: String,
    pub object: String,
    pub action: String,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_permissions)]
pub struct NewPermission<'a> {
    pub creator_user_id: i32,
    pub subject: &'a String,
    pub object: &'a String,
    pub action: &'a String,
}

#[derive(
    Deserialize,
    Serialize,
    Clone,
    Validate,
    Identifiable,
    Queryable,
    Debug,
    Associations,
    Selectable,
)]
#[diesel(belongs_to(Permission))]
#[diesel(table_name = app_permission_conditions)]
pub struct PermissionCondition {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,

    pub creator_user_id: i32,

    #[serde(skip_serializing)]
    pub permission_id: i32,

    pub name: String,
    pub value: String,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = app_permission_conditions)]
pub struct NewPermissionCondition {
    pub creator_user_id: i32,
    pub permission_id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone, Validate, Identifiable, Queryable, Debug, Selectable, Associations)]
#[diesel(table_name = translations)]
#[diesel(belongs_to(Account, foreign_key = translator_account_id))]
pub struct Translation {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,

    #[serde(skip_serializing)]
    pub creator_user_id: i32,

    pub translator_account_id: i32,

    pub mushaf_id: i32,

    pub language: String,
    pub release_date: Option<NaiveDate>,
    pub source: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = translations)]
pub struct NewTranslation {
    pub creator_user_id: i32,
    pub translator_account_id: i32,

    pub language: String,
    pub release_date: Option<NaiveDate>,
    pub source: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Validate, Identifiable, Queryable, Debug, Selectable)]
#[diesel(table_name = translations_text)]
#[diesel(belongs_to(Translation))]
pub struct TranslationText {
    #[serde(skip_serializing)]
    pub id: i32,
    pub uuid: Uuid,

    #[serde(skip_serializing)]
    pub creator_user_id: i32,

    #[serde(skip_serializing)]
    pub translation_id: i32,

    #[serde(skip_serializing)]
    pub ayah_id: i32,

    pub text: String,

    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = translations_text)]
pub struct NewTranslationText<'a> {
    pub creator_user_id: i32,
    pub translation_id: i32,
    pub ayah_id: i32,
    pub text: &'a String,
}
