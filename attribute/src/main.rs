
fn main() {
    println!("Hello, world!");
}

fn print1() -> u32{
    return 1;
}

#[test]
fn test_print1(){
    assert_eq!(print1(), 1);
}
#[test]
#[ignore = "not yet implemented"]
fn test_print2(){
    assert_eq!(print1(), 2);
}

#[test]
#[should_panic(expected = "values don't match")]
fn mytest() {
    assert_eq!(1, 2, "values don't match");
}