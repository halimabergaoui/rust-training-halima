use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
fn main() {
    let mut portfo = PorfolioAccount::new("bacem porfolio ", 0.0f64, 0.0f64);

    let  p: &mut dyn Wallet = &mut portfo;

    p.deposit(2.3);

    p.deposit_multiple(&[ 1.0 ,2.0 ,3.0]);

    println!(" porfolio de Bacem est : {}", p.get_bitcoin());

    let p1: PorfolioAccount<f64> = PorfolioAccount::new("Amal porfolio ", 6.1f64, 8.3f64);

    println!(" porfolio de Amal est : {}", p1.get_bitcoin());

}

pub enum Trainee {
    Bacem(u32),
    Amal(String),
    Jawaher(bool),
}

enum Currency {
    BITCOIN,
    ETHEREUM,
}

struct PorfolioAccount<T: Add<Output = T>> {
    name: String,
    values: [(Currency, T); 2],
}

impl Display for PorfolioAccount<f64>
 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            " Porfolio  : name = {} , BITCOIN = {} , ETH = {}",
            self.name, self.values[0].1, self.values[1].1
        );
        Result::Ok(())
    }
}

impl PorfolioAccount<f64> {
    pub fn new(name: &str, bitcoin: f64, eth: f64) -> PorfolioAccount<f64> {
        PorfolioAccount::<f64>{
            name: name.to_string(),
            values: [(Currency::BITCOIN, bitcoin), (Currency::ETHEREUM, eth)],
        }
    }

}


pub trait Wallet {

    fn deposit(&mut self , value: f64) -> ();

    fn deposit_multiple(&mut self , values: &[f64]) {
        for v in values {
            self.deposit(*v);
        }
    }

    fn widhdraw(&mut self , value: f64) -> ();

    fn get_bitcoin(&self) -> f64;

}

impl Wallet for PorfolioAccount<f64> {
    fn deposit(&mut self , value: f64) -> () {
        let mut old = self.values[0].1;
        old += value;
        self.values[0] = (Currency::BITCOIN, old);
    }

    fn widhdraw(&mut self , value: f64) -> () {
        let mut old = self.values[0].1;
        old -= value;
        self.values[0] = (Currency::BITCOIN, old);
    }

    fn get_bitcoin(&self) -> f64 {
        self.values[0].1
    }

} 
