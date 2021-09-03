use std::collections::HashMap;

pub fn group_by<'a, A, B, C, F>(list: &[(&'a A, B)], by: F) -> HashMap<C, Vec<(&'a A, B)>>
where
    A: ?Sized,
    B: Copy,
    C: Copy + Eq + std::hash::Hash,
    F: Fn(&A) -> C,
{
    let mut result: HashMap<C, Vec<(&A, B)>> = HashMap::new();
    for (a, b) in list {
        let c = by(a);
        let values = match result.get_mut(&c) {
            Some(v) => v,
            None => {
                result.insert(c, Vec::new());
                result.get_mut(&c).expect("Must be here")
            }
        };
        values.push((a, *b));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_by_str() {
        let list = [("A", 10_u8), ("XA", 30), ("B", 20), ("XB", 40)];
        let result = group_by(&list, |e| e.len());
        assert_eq!(result.len(), 2);
        assert_eq!(result[&1], vec![("A", 10), ("B", 20)]);
        assert_eq!(result[&2], vec![("XA", 30), ("XB", 40)]);
    }

    #[test]
    fn group_by_int() {
        let list = [(&1_u8, 10_u8), (&2, 20), (&3, 30), (&4, 40)];
        let result = group_by(&list, |e| e % 2 == 0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[&true], vec![(&2, 20), (&4, 40)]);
        assert_eq!(result[&false], vec![(&1, 10), (&3, 30)]);
    }
}
