use rand::prelude::*;

pub struct DiceRoll{
    pub rolls: usize,
    pub dice: usize,
    pub bonus: usize,
}

impl DiceRoll {
    pub fn new_from_string(string: &str) -> DiceRoll {
        //We assume if there's no plus, the string has no bonus
        let (dice_string, bonus) = string.split_once("+").unwrap_or((string, "0"));
        let (rolls, dice) = dice_string.split_once("d").unwrap();
        let (rolls, dice, bonus) = (rolls.parse::<usize>().unwrap(), dice.parse::<usize>().unwrap(), bonus.parse::<usize>().unwrap());

        DiceRoll {rolls, dice, bonus}
    }

    pub fn to_string(&self) -> String {
        String::from(format!("{}d{}+{}", self.rolls, self.dice, self.bonus))
    }

    pub fn roll(&self) -> usize {
        let mut final_value = 0;
        for _ in 0..self.rolls {
            final_value += (thread_rng().gen::<usize>() % self.dice) + 1;
        }

        final_value += self.bonus;

        final_value
    }
}