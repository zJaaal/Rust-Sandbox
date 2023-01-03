pub fn freq_seq(s: &str, sep: &str) -> String {
    //This was my first solution

    // let mut char_map: HashMap<char, u32> = HashMap::new();

    // for char in s.chars() {
    //     char_map
    //         .entry(char)
    //         .and_modify(|value| *value += 1)
    //         .or_insert(1);
    // }

    // s.chars()
    //     .map(|char| char_map[&char].to_string())
    //     .collect::<Vec<String>>()
    //     .join(sep);

    //or

    s.chars()
        .map(|c| s.matches(c).count().to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
