pub fn add_length(s: &str) -> Vec<String> {
    s.split(" ").map(|x| format!("{} {}", x, x.len())).collect()
}
