use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
fn main() {
    let mut p: PorfolioAccount<f64> = PorfolioAccount::new("bacem porfolio ", 0.0f64, 0.0f64);

    p.deposit_bitcoin(2.3);

    p.deposit_bitcoin(3.3);

    println!(" porfolio de Bacem est : {}", p);

    let p: PorfolioAccount<f64> = PorfolioAccount::new("Amal porfolio ", 6.1f64, 8.3f64);

    println!(" porfolio de Bacem est : {}", p);
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
    pub fn deposit_bitcoin(&mut self , d: f64) {
        let mut old = self.values[0].1;
        old += d;
        self.values[0] = (Currency::BITCOIN, old);
    }

}
