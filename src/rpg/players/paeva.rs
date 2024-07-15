use crate::rpg::character;
use crate::rpg::players::eponym;

impl eponym::Players {
    pub fn paeva() -> character::Character {
        let attributes = character::Attributes::from_vec(vec![8, 12, 11, 9, 9, 11, 11, 9]);
        character::Character::new(attributes)
    }
}
