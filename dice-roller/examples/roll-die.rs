#[macro_use]
extern crate log;
use dice_roller::Die;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!("Starting roll die example");
    let d6 = Die { number_sides: 6 };
    println!("Result of die roll: {}", d6.roll_die().to_string());
}
