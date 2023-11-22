use serde::{Deserialize, Serialize};

/// Order the result list by ASC or DESC
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    // ASC
    Asc,

    // DESC
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Self::Asc
    }
}

pub trait Filters {
    fn sort(&self) -> Option<String>;
    fn order(&self) -> Option<Order>;
    fn from(&self) -> Option<u64>;
    fn to(&self) -> Option<u64>;
}

/// This trait will be used to impl to the nq-api models
///
/// You can use Vec<Model> or T as a diesel query
/// the best way to use this trait is to impl it to T as diesel query
/// and you get query result at the end
pub trait Filter {
    type Output;

    fn filter(filters: Box<dyn Filters>) -> Self::Output;
}
