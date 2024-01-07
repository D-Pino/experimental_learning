use std::collections::HashMap;

pub fn flatten_json() {
    let mut my_dict = HashMap::new();
    my_dict.insert("key1", "value1");
    print!("{my_dict:?}")
}