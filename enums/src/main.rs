fn main() {
    //enum
    let trainer: Trainee = Trainee::Bacem(9u32);
    let trainee1: Trainee = Trainee::Amal("Amal2".to_string());
    let trainee2: Trainee = Trainee::Jawaher(true);

    let t1 = &trainee1;

    use Trainee::*;
    let a = match t1 {
        //pattern
        Bacem(9) => 9u32,
        Bacem(x) => *x,
        Amal(x) => x.len() as u32,
        _ => {
            println!(" I dont know");
            10u32
        }
    };

    println!(" a= {}", a);
}
pub enum Trainee {
    Bacem(u32),
    Amal(String),
    Jawaher(bool),
}
