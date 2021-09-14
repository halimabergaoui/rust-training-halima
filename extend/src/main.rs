fn main() {
    let mut numbers:Vec<u32> = vec![ 1 , 2 , 5 , 7 , 10];
    numbers.extend(0..5);

    for i in &numbers {
        println!("{} " , i);
    }
}
