#[macro_use]
extern crate log;
use dice_roller::Dice;
use dice_roller::Die;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!("Starting roll dice example");

    let mut dice: Dice = Default::default();

    let d6 = Die { number_sides: 6 };
    dice.add_die(d6).unwrap();

    let d10 = Die { number_sides: 10 };
    dice.add_die(d10).unwrap();

    println!("Result of dice roll: {}", dice.roll_dice());
}
