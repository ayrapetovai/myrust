use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// for types with don't impl Copy trait keys/values will be moved to HashMap
// HashMap will be owner of thous keys/values

#[derive(Debug)]
struct Verbose {
    id: i32
}

impl Drop for Verbose{
    fn drop(&mut self) {
        println!("Dropping Verbose{{ id: {} }}", self.id);
    }
}

impl PartialEq for Verbose {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Verbose {}

impl Hash for Verbose {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

fn main() {
    {
        let mut scores = HashMap::new(); // an empty hash map
        // * scores.insert("Yellow".to_string(), 10);
        // * scores.insert("Blue".to_string(), 50);
        scores.insert("Yellow", 10);
        scores.insert("Blue", 50);

        let command_name = "Yellow".to_string();
        // * if let Some(some_ones_score) = scores.get(&command_name) { // command_name was not invalidated
        if let Some(some_ones_score) = scores.get(&command_name[..]) { // command_name was not invalidated
            println!("HashMap contains value {} for key {}", some_ones_score, command_name);
            println!("Inspected command {}", command_name);
        }

        // updating a value: overriding an existing value
        scores.insert("Blue", 75);
        println!("HashMap with mutated value of Blue is {:?}", scores);
        // updating a value: insert if absent
        scores.entry("Blue").or_insert(75);
        println!("HashMap with mutated value of Blue is {:?}", scores);
    }
    {
        // updating a value: default + modify
        let text = "some long long string with text with no meaning";
        let mut count_words = HashMap::new();
        for word in text.split_whitespace() {
            let counter = count_words.entry(word).or_insert(0); // : &mut i32
            *counter += 1;
        }
        println!("In phrase '{}' word statistics is {:?}", text, count_words);
    }
    {
        let names = vec!["Yellow", "Blue"];
        let initial_scores = vec![10, 50];
        // need to specify type of scores explicitly for to choose right overloaded 'collect'
        let scores: HashMap<_, _> = names.into_iter().zip(initial_scores.into_iter()).collect();
        println!("scores {:?}", scores);
    }
    {
        let mut hm = HashMap::new();
        hm.insert("key", Verbose{ id: 1 });
        println!("Hash map with verbose is {:?}", hm);
        // value is dropped after scope ends
    }
    {
        let mut hm = HashMap::new();
        hm.insert(Verbose{ id: 1 }, 1); // key is moved to hash map
        println!("Hash map with verbose is {:?}", hm);
        // key is dropped after scope ends
    }
    {
        // iterating over hash map
        let mut scores = HashMap::new(); // an empty hash map
        scores.insert("Yellow".to_string(), 10);
        scores.insert("Blue".to_string(), 50);
        for (k, v) in scores.iter() {
            println!("Kye: {}, Value: {}.", k, v); // HashMap does not guarantee traverse in order of insertion
        }
    }
}