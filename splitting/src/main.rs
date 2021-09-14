fn main() {
    println!("************ Splitting **********");

    let v = vec![0,1,2,3,2,4,3,4,5,6,6];
    let a = &v[1];
    let b = &v[2];

    let mid = v.len() / 2;
    let front_half = &v[..mid];
    let back_half = [mid..];


    let mut v1 = vec![0,1,2,3];
    let a = &mut v1[1];
    let b = &mut v1[2];

    //let x = v.iter();
   // let y = v1.iter();
    for i in v1.iter_mut() {
        println!(" {}  ", i);
    }

    println!("********* split_at *******");
    let (res1,res2) = v1.split_at(2) ;
    //println!(" res = {}", res);
    
    for i in res1.iter(){
        println!(" res left = {}  ", i);
    }
    for i in res2.iter(){
        println!(" res right = {}  ", i);
    }

    println!("********* split_first *******");

    if let Some((first, elements)) = v.split_first() {
        println!("{}  ", first);
        for (i, j) in elements.into_iter().enumerate() {
            println!("{} = {}", i, j);
        } 
  
    }

    println!("********* split_last *******");

    if let Some((last, elements)) = v.split_last() {
        println!("{}  ", last);
        for (i, j) in elements.into_iter().enumerate() {
            println!("{} = {}", i, j);
        } 
  
    }

    println!("*********split *******");

    let res = v.split(|m| m==&3u32) ;
      
        for i in res.into_iter() {
            println!("---");
            for j in i.into_iter() {
            println!("{}",  j);
            }
        } 

        println!("*********split n *******");

    let res = v.splitn(2 ,|m| m==&3u32) ;
      
        for i in res.into_iter() {
            println!("---");
            for j in i.into_iter() {
            println!("{}",  j);
            }
        } 
  
        println!("********* rsplit n *******");

        let res = v.rsplitn(2 ,|m| m==&3u32) ;
          
            for i in res.into_iter() {
                println!("---");
                for j in i.into_iter() {
                println!("{}",  j);
                }
            } 


            println!("********* chunks *******");

            let res = v.chunks(4) ;
              
                for i in res.into_iter() {
                    println!("---");
                    for j in i.into_iter() {
                    println!("{}",  j);
                    }
                } 

                println!("********* windows *******");

            let res = v.windows(4) ;
              
                for i in res.into_iter() {
                    println!("---");
                    for j in i.into_iter() {
                    println!("{}",  j);
                    }
                } 
}
