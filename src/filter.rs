pub struct Sort {}
pub struct Order {}
pub struct From {}
pub struct Limit {}

/// Possible Filters for Database Table
/// This list may change in future updates
pub struct Filter {
    sort: Sort,
    order: Order,
    from: From,
    limit: Limit,
}
