use std::fmt::{Display, Formatter, Result};
fn main() {
    let mut p: PorfolioAccount = PorfolioAccount::new("bacem porfolio ", 0.0, 0.0);

    p.deposit_bitcoin(2.3);

    p.deposit_bitcoin(3.3);

    println!(" porfolio de Bacem est : {}", p);

    let p: PorfolioAccount = PorfolioAccount::new("Amal porfolio ", 6.1, 8.3);

    println!(" porfolio de Bacem est : {}", p);
}

enum Currency {
    BITCOIN,
    ETHEREUM,
}

struct PorfolioAccount {
    name: String,
    values: [(Currency, f64); 2],
}

impl Display for PorfolioAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            " Porfolio  : name = {} , BITCOIN = {} , ETH = {}",
            self.name, self.values[0].1, self.values[1].1
        );
        Result::Ok(())
    }
}

impl PorfolioAccount {
    pub fn new(name: &str, bitcoin: f64, eth: f64) -> PorfolioAccount {
        PorfolioAccount {
            name: name.to_string(),
            values: [(Currency::BITCOIN, bitcoin), (Currency::ETHEREUM, eth)],
        }
    }

    pub fn deposit_bitcoin(&mut self , d: f64) {
        let mut old = self.values[0].1;
        old += d;
        self.values[0] = (Currency::BITCOIN, old);
    }
}

pub struct Employee(String , String);

pub struct Sqoin;
