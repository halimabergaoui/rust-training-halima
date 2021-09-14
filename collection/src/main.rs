fn main() {
    let mut v = Vec::<u32>::with_capacity(10);
    let mut v2 = Vec::<u32>::with_capacity(10);

    v.extend(1..14);

    let boxes: Vec<Box<&u32>> = (&v).into_iter().map(|a| Box::new(a)).collect();

    let x = v[0];

    let b = &boxes[0];

    let vclone = (&v).clone();

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    {
        let mut sliceref = &(&v)[5..11];
        println!("-----------");

        for (i, j) in sliceref.into_iter().enumerate() {
            println!(" {} = {} ", i, j);
        }

        println!(" first is {}", sliceref.first().unwrap_or(&0u32));
        println!(" last is {}", sliceref.last().unwrap_or(&0u32));
        println!(" number 4 is {}", sliceref.get(4).unwrap_or(&0u32));
    }
    v.push(77u32);

    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    v.pop();

    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    v.insert(0 , 77u32);
    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    v.drain(1..4);
    println!("-----------");

    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    let table = [ [1,2 , 77] , [3,4 , 88] , [5,6 , 99] ].join(&15);
    for (i, j) in (&table).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    use std::collections::VecDeque;
    let mut vq: VecDeque::<u32> = VecDeque::new();
    vq.push_front(10u32);
println!("**********************");
    v2.extend(1..5);
    v.append(&mut v2);
    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!("************DRAIN**********");

    let res = v.drain(0..3);
    for (i, j) in res.into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!("**** V ***");
    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

    println!("************ retrain **********");
    v.retain(|x| x<&5);
    for (i, j) in (&v).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }
    println!();
    println!("************ dedup **********");
let mut byte_vec = [1,2,2,3,3,5,4].to_vec();
    {byte_vec.dedup();
    for i in (&byte_vec).into_iter(){
        print!(" {}  ", i);
    }
    }
    println!();
    println!("************ dedup-by **********");
    byte_vec.dedup_by(|x,y| x==y);
    for i in (&byte_vec).into_iter(){
        print!(" {}  ", i);
    }
    println!();
  
    println!("************ dedup-by_key **********");

   let s :Sqoin=Sqoin{nom:"jawaher".to_string(),cin:1};
   let s2 :Sqoin=Sqoin{nom:"jawaher".to_string(),cin:1};
   let s3 :Sqoin=Sqoin{nom:"jawher".to_string(),cin:2};
   let s4 :Sqoin=Sqoin{nom:"jawaher".to_string(),cin:2};
   let mut struct_vec = [&s,&s2,&s3,&s4].to_vec();

    struct_vec.dedup_by_key(|x| x.cin);
    for i in struct_vec.into_iter(){
        println!(" {}  ", i.cin );
    }

    println!("************ concat **********");
    let table = [ [1,2 , 77] , [3,4 , 88] , [5,6 , 99] ].concat();
    for (i, j) in (&table).into_iter().enumerate() {
        println!(" {} = {} ", i, j);
    }

 
    use std::collections::LinkedList;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::BTreeMap;
    use std::collections::HashSet;
    use std::collections::BTreeSet;

}

pub struct Sqoin {
    nom:String ,
    cin:u64
}