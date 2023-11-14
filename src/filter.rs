use serde::{Deserialize, Serialize};

/// Sort (SortBy)
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    /// Sort by number
    Number,

    /// Sort by Name
    /// Returns sorted T
    Name,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Number
    }
}

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

/// Possible Filters for Database Table

/// This list may change in future updates
#[derive(Deserialize, Serialize)]
pub struct BaseFilters {
    pub sort: Option<Sort>,
    pub order: Option<Order>,

    pub from: Option<u64>,
    pub to: Option<u64>,
}

impl Filters for BaseFilters {
    fn sort(&self) -> Option<Sort> {
        self.sort.clone()
    }

    fn order(&self) -> Option<Order> {
        self.order.clone()
    }

    fn from(&self) -> Option<u64> {
        self.from
    }

    fn to(&self) -> Option<u64> {
        self.to
    }
}

pub trait Filters {
    fn sort(&self) -> Option<Sort>;
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
