pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self {
            Player { health: 0, mana: None, .. } => Some(Player { health: 100, mana: None, ..*self }),
            Player { health: 0, mana: Some(_), .. } => Some(Player { health: 100, mana: Some(100), ..*self }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    return 0;

                } else {
                    self.mana = Some(mana - mana_cost);
                    2 * mana_cost
                }
            },
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;

                } else {
                    self.health = 0;

                }
                0
            }
        }
    }
}