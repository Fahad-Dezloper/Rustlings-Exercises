// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.
use std::io::{self, Read};

mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use self::fruits::PEAR as FRUIT_PEAR;
    pub use self::veggies::CUCUMBER as VEGGIE_CUCUMBER;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::FRUIT_PEAR,
        delicious_snacks::VEGGIE_CUCUMBER,
    );
}
