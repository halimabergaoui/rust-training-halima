fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];

    println!("************ take *************");

    let iter = rust_group.iter().take(3);
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ take while *************");

    let iter = rust_group.iter().take_while(|m| m.len() > 4);
    for m1 in iter {
        println!("{}", m1);
    }
}
