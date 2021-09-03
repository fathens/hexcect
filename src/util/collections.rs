use std::collections::HashMap;
use std::hash::Hash;

pub trait DivideList {
    type T;

    fn divide_by<K, F>(&self, by: F) -> HashMap<K, Vec<&Self::T>>
    where
        K: Copy + Eq + Hash,
        F: Fn(&Self::T) -> K;
}

impl<T> DivideList for [T] {
    type T = T;

    fn divide_by<K, F>(&self, by: F) -> HashMap<K, Vec<&T>>
    where
        K: Copy + Eq + Hash,
        F: Fn(&T) -> K,
    {
        let mut result: HashMap<K, Vec<&T>> = HashMap::new();
        for t in self {
            let c = by(t);
            let values = match result.get_mut(&c) {
                Some(v) => v,
                None => {
                    result.insert(c, Vec::new());
                    result.get_mut(&c).expect("Must be here")
                }
            };
            values.push(t);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gather_by_str() {
        let list = [("A", 10_u8), ("XA", 30), ("B", 20), ("XB", 40)];
        let result = list.divide_by(|(s, _)| s.len());
        assert_eq!(result.len(), 2);
        assert_eq!(result[&1], vec![&("A", 10), &("B", 20)]);
        assert_eq!(result[&2], vec![&("XA", 30), &("XB", 40)]);
    }

    #[test]
    fn gather_by_int() {
        let list = [(&1_u8, 10_u8), (&2, 20), (&3, 30), (&4, 40)];
        let result = list.divide_by(|&(e, _)| e % 2 == 0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[&true], vec![&(&2, 20), &(&4, 40)]);
        assert_eq!(result[&false], vec![&(&1, 10), &(&3, 30)]);
    }
}
