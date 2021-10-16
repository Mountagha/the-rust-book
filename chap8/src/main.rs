use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is is {}", third),
        None => println!("There is no third element!"),
    }

    for i in &v {
        println!("{}",i);
    }

    let mut v = vec![3, 4, 8, 10];
    for i in &mut v {
        *i += 5;
    }

    // a hack to use multiple types in a vector
    #[derive(Debug)]
    enum Multiple {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let data = vec![
        Multiple::Int(2),
        Multiple::Float(4.0),
        Multiple::Text(String::from("test")),
    ];

    println!("{:?}", data);

    // Strings
    let mut s = "starting".to_string();
    s.push_str(" from the bottom");
    s.push('!');

    let hello = String::from("Здравствуйте");

    let s = &hello[0..4];
    println!("{}", s);

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("Yellow"), 15);

    let team_name = String::from("blue");
    match scores.get(&team_name) {
        Some(score) => println!("Retrieved value : {}", score),
        None => println!("No value corresponding to this key"),
    }

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    // we gonna count world in the next string

    let text = "On va compter les mots dans cette phrase qui se repte ici compter les les";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


}


