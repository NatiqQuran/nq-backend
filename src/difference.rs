use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// Returns the key of an object
/// Kind of the name of *existing* object
///
/// Mostly used to identify these items in hashmap's
pub trait GetKey {
    fn get_key(&self) -> String
    where
        Self: Sized;
}

/// Holds the data that Difference Object Can read
///
/// and simplify the Difference Object Usage
pub struct DifferenceContext<T>
where
    T: GetKey,
{
    target: HashMap<String, T>,
    new: Vec<(String, T)>,
}

impl<T> DifferenceContext<T>
where
    T: GetKey,
{
    /// Creates a new Context
    ///
    /// Only needs a vec of T where T impliments the GetKey trait
    /// so the context can turn the target Vec into HashMap, and
    /// new Vec to type Vec<(String,T)>
    pub fn new(target: Vec<T>, new: Vec<T>) -> Self {
        let mut target_map: HashMap<String, T> = HashMap::new();

        for item in target {
            target_map.insert(item.get_key(), item);
        }

        let new_difference_compatible: Vec<(String, T)> =
            new.into_iter().map(|item| (item.get_key(), item)).collect();

        Self {
            target: target_map,
            new: new_difference_compatible,
        }
    }
}

/// The final result of Diff
///
/// So the dev can know When to update
/// ,insert or delete item
#[derive(Debug, PartialEq)]
pub enum DifferenceResult<T> {
    /// Update the existing Data
    ///
    /// First T is old  second is
    /// the update (new)
    Update(T, T),

    /// New Data
    Insert(T),

    /// Removed Data
    Remove(T),
}

/// Finds the difference between Two Vectors
///
/// You can use new function to construct but its better
/// to use DifferenceContext for cleaner code and avoid
/// any confusion
pub struct Difference<T>
where
    T: Hash + Eq + PartialEq,
{
    // Internal state of Difference
    // This Data types makes easier to compute
    // for Difference Object
    
    target: HashMap<String, T>,
    new: Vec<(String, T)>,
}

impl<T> Difference<T>
where
    T: Hash + Eq + PartialEq + Sized + Clone,
{
    /// Creates a new Diff Object
    pub fn new(target: HashMap<String, T>, new: Vec<(String, T)>) -> Self {
        Self { new, target }
    }

    /// Found's the difference between two seprate Vec's
    pub fn diff(&mut self) -> Vec<DifferenceResult<T>> {
        let mut result = vec![];

        for (name, object) in self.new.iter().cloned() {
            match self.target.remove(&name) {
                Some(i) => {
                    // Check if objects in equal (checking hash)
                    if i != object {
                        // If not then this is a update state
                        result.push(DifferenceResult::Update(i, object));
                    }

                    // Else we ignore it (there is no change)
                }

                // then this is a new data
                None => result.push(DifferenceResult::Insert(object)),
            }
        }

        // Get the remaining data from target and turn them into
        // DifferenceResult
        //
        // remaining data means we found the update and insert, when remains
        // must be removed
        let remaining_as_remove: Vec<DifferenceResult<T>> = self
            .target
            .to_owned()
            .into_values()
            .map(|item| DifferenceResult::Remove(item))
            .collect();

        result.extend(remaining_as_remove);

        result
    }
}

/// Construct Difference from DifferenceContext
///
/// For clearner code
impl<T> From<DifferenceContext<T>> for Difference<T>
where
    T: Hash + Eq + PartialEq + Sized + Clone + GetKey,
{
    fn from(value: DifferenceContext<T>) -> Self {
        Self::new(value.target, value.new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Hash, Eq, PartialEq, Debug, PartialOrd, Clone)]
    struct TestData<'a> {
        name: &'a str,
        value: &'a str,
    }

    impl<'a> GetKey for TestData<'a> {
        fn get_key(&self) -> String {
            self.name.to_string()
        }
    }

    #[test]
    fn test_diff_with_update_insert_remove() {
        let target = vec![
            TestData {
                name: "Hello",
                value: "true",
            },
            TestData {
                name: "OOK",
                value: "OOK",
            },
        ];

        let new = vec![
            TestData {
                name: "Hello",
                value: "false",
            },
            TestData {
                name: "Ok",
                value: "test",
            },
        ];

        let context = DifferenceContext::new(target, new);
        let mut diff = Difference::from(context);
        let result = diff.diff();

        let expected = vec![
            DifferenceResult::Update(
                TestData {
                    name: "Hello",
                    value: "true",
                },
                TestData {
                    name: "Hello",
                    value: "false",
                },
            ),
            DifferenceResult::Insert(TestData {
                name: "Ok",
                value: "test",
            }),
            DifferenceResult::Remove(TestData {
                name: "OOK",
                value: "OOK",
            }),
        ];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_diff_with_insert() {
        let target = vec![];

        let new = vec![TestData {
            name: "Hello",
            value: "false",
        }];

        let context = DifferenceContext::new(target, new);
        let mut diff = Difference::from(context);
        let result = diff.diff();

        let expected = vec![DifferenceResult::Insert(TestData {
            name: "Hello",
            value: "false",
        })];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_diff_with_update() {
        let target = vec![TestData {
            name: "Hello",
            value: "false",
        }];

        let new = vec![TestData {
            name: "Hello",
            value: "true",
        }];

        let context = DifferenceContext::new(target, new);
        let mut diff = Difference::from(context);
        let result = diff.diff();

        let expected = vec![DifferenceResult::Update(
            TestData {
                name: "Hello",
                value: "false",
            },
            TestData {
                name: "Hello",
                value: "true",
            },
        )];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_diff_with_remove() {
        let target = vec![TestData {
            name: "Hello",
            value: "false",
        }];

        let new = vec![];

        let context = DifferenceContext::new(target, new);
        let mut diff = Difference::from(context);
        let result = diff.diff();

        let expected = vec![DifferenceResult::Remove(TestData {
            name: "Hello",
            value: "false",
        })];

        assert_eq!(expected, result);
    }
}
