fn main() {
    let mut vec1: Vec<u32> = Vec::new();
    vec1.push(10);
    vec1.push(11);

    for i in vec1 {
        print!(" {} ", i);
    }

    println!();
}
