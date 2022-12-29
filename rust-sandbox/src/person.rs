struct Person {
    name: String,
    middle_name: Option<String>,
    last_name: String,
}

pub fn main() {
    let build_full_name = |person: &Person| -> String {
        let mut full_name: String = String::new();

        full_name.push_str(&person.name);
        full_name.push_str(" ");

        if let Some(middle_name) = &person.middle_name {
            full_name.push_str(&middle_name);
            full_name.push_str(" ");
        }

        full_name.push_str(&person.last_name);
        full_name
    };

    let john = Person {
        name: String::from("James"),
        middle_name: Some(String::from("Oliver")),
        last_name: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");
    println!("name: {}", build_full_name(&john));

    let alice = Person {
        name: String::from("Alice"),
        middle_name: None,
        last_name: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");
    println!("name: {}", build_full_name(&alice));

    let bob = Person {
        name: String::from("Robert"),
        middle_name: Some(String::from("Murdock")),
        last_name: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
    println!("name: {}", build_full_name(&bob));
}
