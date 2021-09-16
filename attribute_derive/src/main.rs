fn main() {

    let foo:Foo<String> = Foo::<String> {
        a:1i32,
        b:"ok".to_string()
    };
    let foo2:Foo<String> = Foo::<String> {
        a:1i32,
        b:"ok".to_string()
    };
    println!("{} , {}", foo.a, foo.b);
    let res=foo.clone();
    let res1= foo==foo2;
}

#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}