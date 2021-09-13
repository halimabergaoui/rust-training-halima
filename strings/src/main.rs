fn main() {
    let s = r"b\acem";
    let s1 = "b\\acem".to_string();
    let mut s2 = String::new();
    s2 += "halima";
    //ascii
    let s3=b"ascii";
    //s1 and s2 are non copy so they should be cloned not moved
    let a = s1.clone();
    let b = s2.clone();
    println!("{} {} {} {} {}", s, s1, s2, a, b);
}
