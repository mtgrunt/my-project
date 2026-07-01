use std::collections::HashMap;

pub fn demo() {
    creating();
    accessing();
    iterating();
    ownership();
    overwriting();
    entry_api();
    update_from_old_value();
}

fn creating() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("[8.3] New HashMap: {scores:?}");
}

fn accessing() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("[8.3] Blue score via .get().copied().unwrap_or(0): {score}");

    let missing = scores.get("Green").copied().unwrap_or(0);
    println!("[8.3] Green (missing) via unwrap_or(0): {missing}");
}

fn iterating() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("[8.3] Iterating (order arbitrary):");
    for (key, value) in &scores {
        println!("      {key}: {value}");
    }
}

fn ownership() {
    let name = String::from("Favorite color");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(name, value);
    // name and value are moved — using them here would be a compile error
    println!("[8.3] Strings moved into map: {map:?}");
}

fn overwriting() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("[8.3] Overwritten Blue: {scores:?}");
}

fn entry_api() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // inserted
    scores.entry(String::from("Blue")).or_insert(50);   // Blue exists, no change
    println!("[8.3] After entry().or_insert(): {scores:?}");
}

fn update_from_old_value() {
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("[8.3] Word counts: {map:?}");
}
