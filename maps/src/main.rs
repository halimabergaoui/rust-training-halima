fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];

    let y = rust_group.iter().map(|m| (String::new() + m + " kind"));
    for member in y {
        println!("{}", member);
    }
}
