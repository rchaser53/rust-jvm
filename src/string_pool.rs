use std::collections::HashMap;

pub struct StringPool {
    pub id: usize,
    pub key_value_map: HashMap<usize, String>,
    pub value_key_map: HashMap<String, usize>,
}

impl StringPool {
    pub fn new() -> StringPool {
        StringPool {
            id: 0,
            key_value_map: HashMap::new(),
            value_key_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: String) -> usize {
        if let Some(id) = self.value_key_map.get(&value) {
            return *id;
        }

        let id = self.id;
        self.id += 1;

        self.value_key_map.insert(value.clone(), id);
        self.key_value_map.insert(id, value);

        id
    }

    pub fn get_value(&self, id: &usize) -> String {
        self.key_value_map
            .get(id)
            .expect("exist string in string_pool")
            .to_string()
    }
}
