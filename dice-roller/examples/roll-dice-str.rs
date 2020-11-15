#[macro_use]
extern crate log;
use dice_roller::Dice;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!("Starting roll dice from str example");

    let mut dice: Dice = Default::default();
    let dice_list = ["1d6", "4d8", "2d10"];
    dice.add_dice(&dice_list).unwrap();

    println!("Result of dice roll: {}", dice.roll_dice());
}
