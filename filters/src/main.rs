fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let y = rust_group.iter().filter(|m| m.len() <= 5);

    for member in y {
        println!("{}", member);
    }
}
