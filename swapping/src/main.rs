fn main() {
    println!("-------- Swap---------");
    let mut vec = [8, 2, 3, 5, 7,1];
    vec.swap(1, 3);
    for (i, j) in vec.iter().enumerate() {
        println!("{} = {}", i, j);
    }
    println!("-------- Swap-Remove ---------");
    let mut v = vec!["bacem", "karima", "khouloud", "jawaher"];
    v.swap_remove(2);
    for (i, j) in v.iter().enumerate() {
        println!("{} = {}", i, j);
    }
    println!("********* Sorting and Searching *******");
    println!("-------- sort---------");
    let mut v1 = vec![0, 1, 2, 3];   
    v1.sort();
    for (i, j) in v1.iter().enumerate() {
        println!("{} = {}", i, j);
    }}
