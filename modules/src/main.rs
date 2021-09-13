mod sqoin_module {
    pub mod sqoin_sub_module {
        pub struct Sqoiner {
            pub number: i32,
        }

        pub trait SqoinRules {
            fn shownumber(sqoiner: &Sqoiner);
        }

        pub fn print_sqoiner(sqoiner: &Sqoiner) -> () {
            println!(" Printing Sqoin {}", sqoiner.number);
        }
    }
}

mod karima {

    pub fn say_hello() {
        println!(" I am Karima");
    }
}

mod jawaher;
mod khouloud;
fn main() {
    sqoin_module::sqoin_sub_module::print_sqoiner(&sqoin_module::sqoin_sub_module::Sqoiner {
        number: 10,
    });
    karima::say_hello();
    khouloud::say_hello();
    jawaher::jawaherImpl::say_hello();
}
