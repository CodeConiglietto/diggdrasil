use specs::{Component, Entity, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct AttackComponent {
    pub attack_roll: DiceRoll,
    pub attack_type: AttackType,
}