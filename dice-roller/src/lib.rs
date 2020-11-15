use rand::distributions::Uniform;
use rand::Rng;

/// This magic number comes from the following computation:
/// * A die has a maximum of 255 sides.
/// * The biggest dice roll is 2^32 - 1 (roll_result is u32) = 4294967295.
/// * The worst case scenario is that all the dice rolled give their max value = 255.
/// -> The max number of dice we can roll is 4294967295 / 255 = 16843009
const MAX_NUMBER_DICE: usize = std::u32::MAX as usize / std::u8::MAX as usize;

/// A single die characterized by its number of sides.
///
/// The maximum number of sides is 255.
#[derive(Debug, PartialEq)]
pub struct Die {
    pub number_sides: u8,
}

impl Die {
    /// Rolls a die once.
    pub fn roll_die(&self) -> u8 {
        let rng = rand::thread_rng();
        let die_range = Uniform::new_inclusive(1, self.number_sides);
        let mut roll_die = rng.sample_iter(&die_range);
        roll_die.next().unwrap()
    }
}

/// A set of dice.
///
/// There is maximum amount of dice a set can contain is 16843009 (MAX_NUMBER_DICE constant).
#[derive(Default, PartialEq)]
pub struct Dice {
    pub dice: Vec<Die>,
}

impl Dice {
    /// Adds a die to the current set of dice.
    pub fn add_die(&mut self, die: Die) -> Result<(), &'static str> {
        if self.dice.len() < MAX_NUMBER_DICE {
            log::info!("Adding 1d{}", &die.number_sides);
            self.dice.push(die);
        } else {
            return Err("Maximum amount of dice reached");
        }
        Ok(())
    }

    /// Parses argument and returns a tuple with the number of dice of a specific type.
    fn parse_add_args(arg: &str) -> Result<(u8, u8), &'static str> {
        let parsed_arg: Vec<&str> = arg.split(|c| c == 'd' || c == 'D').collect();
        if parsed_arg.len() != 2 {
            return Err("Argument malformed, too many characters");
        }
        let number_dice: u8 = parsed_arg[0]
            .parse()
            .expect("Argument malformed, left side of separator is not an int");
        let number_sides: u8 = parsed_arg[1]
            .parse()
            .expect("Argument malformed, right side of separator is not an int");
        Ok((number_dice, number_sides))
    }

    /// Adds multiple dice to the dice set.
    pub fn add_dice(&mut self, args: &[&str]) -> Result<(), &'static str> {
        for arg in args {
            let dice_args = Dice::parse_add_args(arg)?;
            log::info!("Adding {}d{} to set", dice_args.0, dice_args.1);
            for _ in 0..dice_args.0 {
                self.add_die(Die {
                    number_sides: dice_args.1,
                })?;
            }
        }
        Ok(())
    }

    /// Rolls all dice in the set and returns the sum of the rolls.
    pub fn roll_dice(&mut self) -> u32 {
        let mut roll_result: u32 = 0;
        for die in &self.dice {
            roll_result += die.roll_die() as u32;
        }
        roll_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_die() {
        let die = Die { number_sides: 6 };
        let mut dice: Dice = Default::default();
        dice.add_die(die).unwrap();
        assert_eq!(dice.dice[0], Die { number_sides: 6 });
    }

    #[test]
    fn add_dice() {
        let args = ["1d6", "2D3", "3d100"];
        let mut dice: Dice = Default::default();
        dice.add_dice(&args).unwrap();
        assert_eq!(dice.dice[0], Die { number_sides: 6 });
        assert_eq!(dice.dice[1], Die { number_sides: 3 });
        assert_eq!(dice.dice[2], Die { number_sides: 3 });
        assert_eq!(dice.dice[3], Die { number_sides: 100 });
        assert_eq!(dice.dice[4], Die { number_sides: 100 });
        assert_eq!(dice.dice[5], Die { number_sides: 100 });
    }

    #[test]
    #[should_panic(expected = "Argument malformed, left side of separator is not an int")]
    fn add_dice_not_int_first_argument() {
        let not_first_int = ["ad6"];
        let mut dice: Dice = Default::default();
        dice.add_dice(&not_first_int).unwrap();
    }

    #[test]
    #[should_panic(expected = "Argument malformed, right side of separator is not an int")]
    fn add_dice_not_int_second_argument() {
        let not_second_int = ["1da"];
        let mut dice: Dice = Default::default();
        dice.add_dice(&not_second_int).unwrap();
    }

    #[test]
    #[should_panic(expected = "Argument malformed, too many characters")]
    fn add_dice_too_many_arguments() {
        let too_many_args = ["1d6d"];
        let mut dice: Dice = Default::default();
        dice.add_dice(&too_many_args).unwrap();
    }
}
