use crate::models::{QuranAyah, QuranMushaf, QuranSurah, QuranWord, Translation};
use crate::schema::mushafs::{self, BoxedQuery as MushafBoxedQuery};
use crate::schema::quran_ayahs::BoxedQuery as AyahBoxedQuery;
use crate::schema::quran_surahs::BoxedQuery as SurahBoxedQuery;
use crate::schema::quran_words::BoxedQuery as WordBoxedQuery;
use crate::schema::translations::table as translations_table;
use crate::schema::{app_accounts, app_user_names};
use crate::{
    error::RouterError,
    filter::{Filter, Filters, Order},
};
use diesel::helper_types::{InnerJoin, IntoBoxed};
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

                value => Err(RouterError::BadRequest(format!(
                    "Sort value {} is not possible!",
                    value
                ))),
            },

            None => Ok(quran_surahs.internal_into_boxed()),
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

                value => Err(RouterError::BadRequest(format!(
                    "Sort value {} is not possible!",
                    value
                ))),
            },

            None => Ok(quran_ayahs.internal_into_boxed()),
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

                value => Err(RouterError::BadRequest(format!(
                    "Sort value {} is not possible!",
                    value
                ))),
            },

            None => Ok(quran_words.internal_into_boxed()),
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
        use crate::schema::mushafs::dsl::*;

        let mut _query = mushafs.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "name" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => mushafs.order(name.asc()).internal_into_boxed(),
                    Order::Desc => mushafs.order(name.desc()).internal_into_boxed(),
                }),

                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => mushafs.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => mushafs.order(created_at.desc()).internal_into_boxed(),
                }),

                "updateTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => mushafs.order(updated_at.asc()).internal_into_boxed(),
                    Order::Desc => mushafs.order(updated_at.desc()).internal_into_boxed(),
                }),

                value => Err(RouterError::BadRequest(format!(
                    "Sort value {} is not possible!",
                    value
                ))),
            },

            None => Ok(mushafs.internal_into_boxed()),
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
pub type TranslationBoxedQueryType = IntoBoxed<
    'static,
    InnerJoin<
        InnerJoin<translations_table, mushafs::table>,
        InnerJoin<app_accounts::table, app_user_names::table>,
    >,
    Pg,
>;

impl Filter for Translation {
    type Output = Result<TranslationBoxedQueryType, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::app_accounts::dsl::app_accounts;
        use crate::schema::app_user_names::dsl::{
            app_user_names, first_name as user_first_name, last_name as user_last_name,
        };
        use crate::schema::mushafs::dsl::{mushafs, name as mushaf_name};
        use crate::schema::translations::dsl::*;

        let mut _query = translations_table
            .inner_join(mushafs)
            .inner_join(app_accounts.inner_join(app_user_names))
            .into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "createTime" => match filters.order().unwrap_or_default() {
                    Order::Asc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(created_at.asc())
                        .internal_into_boxed()),
                    Order::Desc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(created_at.desc())
                        .internal_into_boxed()),
                },

                "updateTime" => match filters.order().unwrap_or_default() {
                    Order::Asc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(updated_at.asc())
                        .internal_into_boxed()),
                    Order::Desc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(updated_at.desc())
                        .internal_into_boxed()),
                },

                "language" => match filters.order().unwrap_or_default() {
                    Order::Asc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(language.asc())
                        .internal_into_boxed()),
                    Order::Desc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(language.desc())
                        .internal_into_boxed()),
                },

                "mushaf" => match filters.order().unwrap_or_default() {
                    Order::Asc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(mushaf_name.asc())
                        .internal_into_boxed()),

                    Order::Desc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(mushaf_name.desc())
                        .internal_into_boxed()),
                },

                "translator_name" => match filters.order().unwrap_or_default() {
                    Order::Asc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(user_first_name.asc())
                        .internal_into_boxed()),

                    Order::Desc => Ok(translations
                        .inner_join(mushafs)
                        .inner_join(app_accounts.inner_join(app_user_names))
                        .order(user_first_name.desc())
                        .internal_into_boxed()),
                },

                value => Err(RouterError::BadRequest(format!(
                    "Sort value {} is not possible!",
                    value
                ))),
            },

            None => Ok(translations
                .inner_join(mushafs)
                .inner_join(app_accounts.inner_join(app_user_names))
                .into_boxed()),
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
