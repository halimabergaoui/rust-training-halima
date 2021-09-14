fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
   
    println!("************ skip *************");

    let iter = rust_group.iter().skip(3);
    for m1 in iter {
        println!("{}", m1);
    }

    println!("************ skip while *************");

    let iter = rust_group.iter().skip_while(|m| m.len() > 4);
    for m1 in iter {
        println!("{}", m1);
    }

}
