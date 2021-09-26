use rand::prelude::*;

pub struct DiceRoll{
    pub rolls: usize,
    pub dice: usize,
    pub bonus: usize,
}

impl DiceRoll {
    pub fn new_from_string(_string: String) -> DiceRoll {
        todo!();
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