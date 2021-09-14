fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    for i in (&rust_group).into_iter().by_ref() {
        println!("{} ", i );
    }
}
