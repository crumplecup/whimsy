use crate::rpg::character::{Attributes, Character};
use crate::rpg::players::eponym;

impl eponym::Players {
    pub fn paeva() -> Character {
        let attributes = Attributes::from_vec(vec![8, 12, 11, 9, 9, 11, 11, 9]);
        Character::new(attributes)
            .with_name("Paeva")
            .with_player("Shaw")
    }
}
