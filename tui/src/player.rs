use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerPosition {
    Defender,   // Lines 1-8 in B$ array
    Midfielder, // Lines 9-16 in B$ array
    Forward,    // Lines 17-24 in B$ array
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStatus {
    NotOwned,      // C(x) = 0
    Available,     // C(x) = 1
    Owned,         // C(x) = 2
    Substitute,    // C(x) = 3
    Playing,       // C(x) = 4
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub position: PlayerPosition,
    pub style: u8,      // A(x): 1-5, affects position-specific rating
    pub power: u8,      // B(x): 1-20, player's overall strength
    pub status: PlayerStatus, // C(x): ownership/playing status
}

impl Player {
    pub fn new(id: usize, name: String, position: PlayerPosition) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Player {
            id,
            name,
            position,
            style: rng.gen_range(1..=5),
            power: rng.gen_range(1..=20),
            status: PlayerStatus::NotOwned,
        }
    }

    pub fn position_from_id(id: usize) -> PlayerPosition {
        match id {
            0..=7 => PlayerPosition::Defender,
            8..=15 => PlayerPosition::Midfielder,
            _ => PlayerPosition::Forward,
        }
    }

    pub fn market_value(&self, league_level: u8) -> i32 {
        // Original: 5000*(5-N)+5000*A(PZ)
        5000 * (5 - league_level as i32) + 5000 * self.style as i32
    }

    pub fn update_power(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        match self.status {
            PlayerStatus::Substitute => {
                // Substitute/injured players lose power: B(HZ)=B(HZ)-10
                self.power = self.power.saturating_sub(10).max(1);
            }
            PlayerStatus::Playing => {
                // Playing increases power: B(HZ)=B(HZ)+1
                self.power = (self.power + 1).min(20);
            }
            PlayerStatus::Available => {
                // Available players get random power: B(HZ)=INT(RND(1)*15)+C(HZ)*2
                self.power = rng.gen_range(1..=15) + (self.status as u8) * 2;
            }
            _ => {
                // Keep power in bounds
                self.power = self.power.clamp(1, 20);
            }
        }

        // Random fluctuation for high-power players
        if self.power >= 12 && rng.gen_bool(0.5) {
            self.power = self.power.saturating_sub(2).max(1);
        }
    }
}

// Player names from the original game (lines 820-840)
pub const PLAYER_NAMES: [&str; 24] = [
    // Defenders (D)
    "BORDON", "TANCREDI", "NELA", "CABRINI",
    "VIERCHOWOD", "JUNIOR", "PASSARELLA", "TRICELLA",
    // Midfielders (C)
    "WILKINS", "ZICO", "SOUNESS", "BAGNI",
    "MARADONA", "PLATINI", "DOSSENA", "BRADY",
    // Forwards (A)
    "ALTOBELLI", "RUMENIGGE", "GIORDANO", "ROSSI",
    "VIRDIS", "GALDERISI", "HATELEY", "SERENA",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStatusLegacy {
    Injured,    // * in display
    Substitute, // S in display
    Playing,    // G in display
}
