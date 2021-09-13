fn main() {
    let mut table: [u32; 10] = [10; 10];

    for i in 0..10 {
        print!(" {}", table[i]);
    }
    println!();
    for i in 0..10 {
        table[i] = (i + 1) as u32;
    }
    for i in 0..10 {
        print!(" {}", table[i]);
    }
}
