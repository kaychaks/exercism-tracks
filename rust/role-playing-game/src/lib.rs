// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(health: u32, mana: u32, level: u32) -> Self {
        Player {
            health,
            mana: if level < 10 { None } else { Some(mana) },
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(100, 100, self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = if self.health > mana_cost {
                    self.health - mana_cost
                } else {
                    0
                };
                0
            }
            Some(x) => {
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
        }
    }
}
