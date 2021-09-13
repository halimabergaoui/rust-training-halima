use std::ops::Add;
fn main() {
    println!(" sum non generic of integers : {}", sum_non_generic(10u32, 20u32));
    println!(" sum of integers : {}", sum_generic::<u32>(10u32, 20u32));
    println!(" sum of floats : {}", sum_generic(1.2f64, 1.3f64));
}
fn sum_generic<T: Add<Output = T>>(x1: T, x2: T) -> T {
    let sum = x1 + x2;
    sum
}
fn sum_non_generic(x1:u32 , x2: u32) -> u32 {
    let sum = x1 + x2;
    sum
}
