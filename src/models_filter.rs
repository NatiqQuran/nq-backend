use crate::models::{ErrorLog, QuranAyah, QuranMushaf, QuranSurah, QuranWord, Translation};
use crate::schema::app_error_logs::BoxedQuery as AppErrorBoxedQuery;
use crate::schema::quran_ayahs::BoxedQuery as AyahBoxedQuery;
use crate::schema::quran_mushafs::BoxedQuery as MushafBoxedQuery;
use crate::schema::quran_surahs::BoxedQuery as SurahBoxedQuery;
use crate::schema::quran_translations::BoxedQuery as TranslationBoxed;
use crate::schema::quran_words::BoxedQuery as WordBoxedQuery;
use crate::{
    error::RouterError,
    filter::{Filter, Filters, Order},
};
use diesel::pg::Pg;
use diesel::{prelude::*, query_dsl::methods::BoxedDsl};

// # Impl filter trait for QuranSurah
//
// So we can provide filters object and get the list of surahs
//
// These codes is pretty straightforward but can be cleaner in the future
// or maybe we can create a macro to avoid any duplicate code
//
// There is two ways to use the filter trait when we need it
// 1. Impl Filter trait to every Model we need to have a filter function (This will cuase duplicate
//    code most of the time)
// 2. Use some macro to generate the impl for every particular model we need
//
// It's not certain to use the same code for each impl for every model we need to have
// Filter trait implemented, So the macro creation for this trait impl would not be easy thing to
// do.
//
// Also there is a macro simular for what we want in macros.rs file
impl Filter for QuranSurah {
    type Output = Result<SurahBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::quran_surahs::dsl::*;

        let mut _query = quran_surahs.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "name" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_surahs.order(name.asc()).internal_into_boxed(),
                    Order::Desc => quran_surahs.order(name.desc()).internal_into_boxed(),
                }),

                "number" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_surahs.order(number.asc()).internal_into_boxed(),
                    Order::Desc => quran_surahs.order(number.desc()).internal_into_boxed(),
                }),

                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_surahs.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_surahs.order(created_at.desc()).internal_into_boxed(),
                }),

                "updateTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_surahs.order(updated_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_surahs.order(updated_at.desc()).internal_into_boxed(),
                }),

                _ => Err(RouterError::from_predefined(
                    "FILTER_SORT_VALUE_NOT_DEFINED",
                )),
            },

            // Default order by number.asc
            None => Ok(quran_surahs.internal_into_boxed().order(number.asc())),
        }?;

        _query = match filters.to() {
            Some(limit) => _query
                .limit(limit as i64)
                .offset(filters.from().unwrap_or_default() as i64),
            None => _query.offset(filters.from().unwrap_or_default() as i64),
        };

        Ok(_query)
    }
}

impl Filter for QuranAyah {
    type Output = Result<AyahBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::quran_ayahs::dsl::*;

        let mut _query = quran_ayahs.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "number" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_ayahs.order(ayah_number.asc()).internal_into_boxed(),
                    Order::Desc => quran_ayahs.order(ayah_number.desc()).internal_into_boxed(),
                }),

                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_ayahs.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_ayahs.order(created_at.desc()).internal_into_boxed(),
                }),

                "updateTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_ayahs.order(updated_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_ayahs.order(updated_at.desc()).internal_into_boxed(),
                }),

                _ => Err(RouterError::from_predefined(
                    "FILTER_SORT_VALUE_NOT_DEFINED",
                )),
            },

            None => Ok(quran_ayahs.internal_into_boxed().order(ayah_number.asc())),
        }?;

        _query = match filters.to() {
            Some(limit) => _query
                .limit(limit as i64)
                .offset(filters.from().unwrap_or_default() as i64),
            None => _query.offset(filters.from().unwrap_or_default() as i64),
        };

        Ok(_query)
    }
}

impl Filter for QuranWord {
    type Output = Result<WordBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::quran_words::dsl::*;

        let mut _query = quran_words.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_words.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_words.order(created_at.desc()).internal_into_boxed(),
                }),

                "updateTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_words.order(updated_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_words.order(updated_at.desc()).internal_into_boxed(),
                }),

                "word" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_words.order(word.asc()).internal_into_boxed(),
                    Order::Desc => quran_words.order(word.desc()).internal_into_boxed(),
                }),

                _ => Err(RouterError::from_predefined(
                    "FILTER_SORT_VALUE_NOT_DEFINED",
                )),
            },

            None => Ok(quran_words.internal_into_boxed().order(created_at.asc())),
        }?;

        _query = match filters.to() {
            Some(limit) => _query
                .limit(limit as i64)
                .offset(filters.from().unwrap_or_default() as i64),
            None => _query.offset(filters.from().unwrap_or_default() as i64),
        };

        Ok(_query)
    }
}

impl Filter for QuranMushaf {
    type Output = Result<MushafBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::quran_mushafs::dsl::*;

        let mut _query = quran_mushafs.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "name" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_mushafs.order(name.asc()).internal_into_boxed(),
                    Order::Desc => quran_mushafs.order(name.desc()).internal_into_boxed(),
                }),

                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_mushafs.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_mushafs.order(created_at.desc()).internal_into_boxed(),
                }),

                "updateTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_mushafs.order(updated_at.asc()).internal_into_boxed(),
                    Order::Desc => quran_mushafs.order(updated_at.desc()).internal_into_boxed(),
                }),

                _ => Err(RouterError::from_predefined(
                    "FILTER_SORT_VALUE_NOT_DEFINED",
                )),
            },

            None => Ok(quran_mushafs.internal_into_boxed().order(created_at.asc())),
        }?;

        _query = match filters.to() {
            Some(limit) => _query
                .limit(limit as i64)
                .offset(filters.from().unwrap_or_default() as i64),
            None => _query.offset(filters.from().unwrap_or_default() as i64),
        };

        Ok(_query)
    }
}

// translator_name <==> account <==> user_names
//pub type TranslationBoxedQueryType = IntoBoxed<
//    'static,
//    LeftJoinOn<
//        translations_table,
//        InnerJoin<app_accounts::table, app_user_names::table>,
//        helper_types::Eq<app_user_names::account_id, app_accounts::id>,
//    >,
//    Pg,
//>;

impl Filter for Translation {
    type Output = Result<TranslationBoxed<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        //use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id};
        //use crate::schema::app_user_names::dsl::{
        //    account_id, app_user_names, first_name as user_first_name, last_name as user_last_name,
        //};
        //use crate::schema::quran_mushafs::dsl::{mushafs, name as mushaf_name};
        use crate::schema::quran_translations::dsl::*;

        let mut _query = quran_translations
            //.inner_join(app_accounts)
            //.left_join(app_user_names.on(account_id.eq(acc_id)))
            .into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "language" => match filters.order().unwrap_or_default() {
                    Order::Asc => Ok(quran_translations
                        .order(language.asc())
                        .internal_into_boxed()),
                    Order::Desc => Ok(quran_translations
                        .order(language.desc())
                        .internal_into_boxed()),
                },
                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_translations
                        .order(created_at.asc())
                        .internal_into_boxed(),
                    Order::Desc => quran_translations
                        .order(created_at.desc())
                        .internal_into_boxed(),
                }),

                "updateTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => quran_translations
                        .order(updated_at.asc())
                        .internal_into_boxed(),
                    Order::Desc => quran_translations
                        .order(updated_at.desc())
                        .internal_into_boxed(),
                }),

                _ => Err(RouterError::from_predefined(
                    "FILTER_SORT_VALUE_NOT_DEFINED",
                )),
            },

            None => Ok(quran_translations.order(language.asc()).into_boxed()),
        }?;

        _query = match filters.to() {
            Some(limit) => _query
                .limit(limit as i64)
                .offset(filters.from().unwrap_or_default() as i64),
            None => _query.offset(filters.from().unwrap_or_default() as i64),
        };

        Ok(_query)
    }
}

impl Filter for ErrorLog {
    type Output = Result<AppErrorBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::app_error_logs::dsl::*;

        let mut _query = app_error_logs.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => app_error_logs.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => app_error_logs
                        .order(created_at.desc())
                        .internal_into_boxed(),
                }),

                _ => Err(RouterError::from_predefined(
                    "FILTER_SORT_VALUE_NOT_DEFINED",
                )),
            },

            None => Ok(app_error_logs.internal_into_boxed().order(created_at.asc())),
        }?;

        _query = match filters.to() {
            Some(limit) => _query
                .limit(limit as i64)
                .offset(filters.from().unwrap_or_default() as i64),
            None => _query.offset(filters.from().unwrap_or_default() as i64),
        };

        Ok(_query)
    }
}
