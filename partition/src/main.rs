fn main() {
    let rust_group = vec!["Bacem", "Karima", "Amal", "Jawaher", "Halima", "Khouloud"];
    let other_group = vec!["Ameni", "Emna", "Wafa", "Fedia", "Ameni", "Saif" ,  "Tawfik" ];

    let mut chain  = rust_group.iter().chain(other_group.iter());

    let (p1,p2) : (Vec::<&str> ,Vec::<&str> )= chain.partition(|n| n.len() <= 4);

    for i in p1 {
        println!(" partition 1 {}" , i);
    }

    for i in p2 {
        println!(" partition 2 {}" , i);
    }}
