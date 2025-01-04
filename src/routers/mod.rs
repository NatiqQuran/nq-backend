pub mod account;
pub mod error;
pub mod organization;
pub mod permission;
pub mod phrase;
pub mod profile;
pub mod quran;
pub mod translation;
pub mod user;

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// finds the relatives in the vector
/// Vec<(Obj1, Obj2)>
/// This will collect the Obj2 that related to the Obj1 and returns
/// a BTreeMap (We want the elements be in order)
pub fn multip<T, U, F, NT>(vector: Vec<(T, U)>, insert_data_type: F) -> BTreeMap<NT, Vec<U>>
where
    T: Sized + Clone,
    U: Sized + Clone,
    NT: Sized + Eq + Hash + Ord,
    F: Fn(T) -> NT,
{
    let mut map: BTreeMap<NT, Vec<U>> = BTreeMap::new();
    for item in vector {
        map.entry(insert_data_type(item.0))
            .and_modify(|c| c.push(item.1.clone()))
            .or_insert(vec![item.1]);
    }

    map
}

/// a BTreeMap (We want the elements be in order)
pub fn maybe_multip<T, U, F, NT>(
    vector: Vec<(T, Option<U>)>,
    insert_data_type: F,
) -> BTreeMap<NT, Vec<U>>
where
    T: Sized + Clone,
    U: Sized + Clone,
    NT: Sized + Eq + Hash + Ord,
    F: Fn(usize, T) -> NT,
{
    let mut map: BTreeMap<NT, Vec<U>> = BTreeMap::new();
    for (index, item) in vector.into_iter().enumerate() {
        map.entry(insert_data_type(index, item.clone().0))
            .and_modify(|c| {
                if let Some(value) = item.1.clone() {
                    c.push(value.clone())
                }
            })
            .or_insert(match item.1 {
                Some(val) => vec![val],
                None => vec![],
            });
    }

    map
}

pub fn count<T>(data: Vec<T>) -> HashMap<T, u32>
where
    T: Sized + Hash + Eq,
{
    let mut map: HashMap<T, u32> = HashMap::new();
    for v in data {
        map.entry(v).and_modify(|v| *v += 1).or_insert(1);
    }

    map
}
