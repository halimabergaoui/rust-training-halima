fn main() {
    let table = [1, 2, 3, 4, 5, 6];
    let v3 = vec![10u32, 11u32];

    let slice = &table[1..3];

    print_slice(slice);
    println!();
    print_slice(&v3);
}

fn print_slice(v: &[u32]) {
    for i in v {
        print!("{} ", i);
    }
}
