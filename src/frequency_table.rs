use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct FrequencyTable<T> {
    pub table: HashMap<T, i64>,
}

impl<T> FrequencyTable<T>
where
    T: Hash + Eq,
{
    pub fn from_vec(v: Vec<T>) -> Self {
        let mut table = HashMap::new();
        for c in v.into_iter() {
            if let Some(freq) = table.get_mut(&c) {
                *freq += 1;
            } else {
                table.insert(c, 1);
            }
        }
        FrequencyTable { table }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::FrequencyTable;

    #[test]
    fn it_works() {
        // let s = "a b c";
        let v = vec!['a', 'b', 'c'];
        let freq_table = FrequencyTable::from_vec(v);
        let expected = HashMap::from([('a', 1), ('b', 1), ('c', 1)]);
        assert_eq!(freq_table.table, expected);
    }
}
