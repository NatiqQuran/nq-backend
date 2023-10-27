/// Sort (SortBy)
pub enum Sort {
    /// Sort by number
    Number,

    /// Sort by Name
    Name,
}

/// Order the result list by ASC or DESC
pub enum Order {
    // ASC
    Asc,

    // DESC
    Desc,
}

/// Possible Filters for Database Table
/// This list may change in future updates
pub struct Filter {
    pub sort: Sort,
    pub order: Order,
}
