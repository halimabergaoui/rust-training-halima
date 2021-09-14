fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];

    for (i, m) in (&rust_group).into_iter().enumerate() {
        println!("{} {}", i , m);
    }
}
