use crate::models::{QuranMushaf, QuranSurah, Translation};
use crate::schema::mushafs::{table as quran_mushafs_table, BoxedQuery as MushafBoxedQuery};
use crate::schema::quran_surahs::{table as quran_surahs_table, BoxedQuery as SurahBoxedQuery};
use crate::schema::translations::{table as translations_table, BoxedQuery as TranslationBoxedQuery};
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

        let mut _query = quran_surahs_table.into_boxed();

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

impl Filter for QuranMushaf {
    type Output = Result<MushafBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::mushafs::dsl::*;

        let mut _query = quran_mushafs_table.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "name" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => mushafs.order(name.asc()).internal_into_boxed(),
                    Order::Desc => mushafs.order(name.desc()).internal_into_boxed(),
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

impl Filter for Translation {
    type Output = Result<TranslationBoxedQuery<'static, Pg>, RouterError>;

    fn filter(filters: Box<dyn Filters>) -> Self::Output {
        use crate::schema::translations::dsl::*;

        let mut _query = translations_table.into_boxed();

        _query = match filters.sort() {
            Some(sort_str) => match sort_str.as_str() {
                "createTime" => Ok(match filters.order().unwrap_or_default() {
                    Order::Asc => translations.order(created_at.asc()).internal_into_boxed(),
                    Order::Desc => translations.order(created_at.desc()).internal_into_boxed(),
                }),

                value => Err(RouterError::BadRequest(format!(
                    "Sort value {} is not possible!",
                    value
                ))),
            },

            None => Ok(translations.internal_into_boxed()),
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
