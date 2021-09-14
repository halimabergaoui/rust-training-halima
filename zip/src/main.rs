fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let other_group = vec!["Ameni", "Emna", "Wafa", "Fedia", "Ameni", "Saif" ,  "Tawfik" ];
    let mut chain  = rust_group.iter().zip(other_group.iter());
    while let Some(v) = chain.next() {
        println!("{} {}", v.0 , v.1 );
    }
}
