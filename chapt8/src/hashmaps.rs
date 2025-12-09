use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    println!("{:#?}", scores);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    // HashMap<_, _> is required for compiler to understnad
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (k, v) in &scores {
        println!("{}: {}", k, v)
    }

    // Access values in a Hash Map
    let score = scores.get(&String::from("blue")).copied().unwrap_or(&0);
    _ = score;

    // entry() is used for insert if not exists
    let mut scores = HashMap::new();

    scores.insert(String::from("green"), 15);

    scores.entry(String::from("green")).or_insert(35);
    scores.entry(String::from("red")).or_insert(25);

    // Ownership of "fav_col" is transferred to the map
    let fav_col = String::from("violet");
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("favorite_color"), fav_col);

    // Update a Hash Map based on an existing value
    update_hm_existing_values()
}

fn update_hm_existing_values() {
    let text = "здравствуй, мир, чудесный мир";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert method returns a &mut V
        let count = map.entry(word.replace(",", "")).or_insert(0);
        *count += 1
    }

    println!("{:#?}", map)
}
