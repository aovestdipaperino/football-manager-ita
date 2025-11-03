use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: usize,
    pub name: String,
    pub league_level: u8,  // N: 1=Serie A, 2=Serie B, 3=Serie C1, 4=Serie C2
    pub wins: u8,
    pub draws: u8,
    pub losses: u8,
    pub goals_for: u16,    // F(x)
    pub goals_against: u16, // E(x)
    pub points: u16,        // G(x)
}

impl Team {
    pub fn new(id: usize, name: String, league_level: u8) -> Self {
        Team {
            id,
            name,
            league_level,
            wins: 0,
            draws: 0,
            losses: 0,
            goals_for: 0,
            goals_against: 0,
            points: 0,
        }
    }

    pub fn goal_difference(&self) -> i32 {
        self.goals_for as i32 - self.goals_against as i32
    }

    pub fn reset_season(&mut self) {
        self.wins = 0;
        self.draws = 0;
        self.losses = 0;
        self.goals_for = 0;
        self.goals_against = 0;
        self.points = 0;
    }

    pub fn record_result(&mut self, goals_for: u16, goals_against: u16) {
        self.goals_for += goals_for;
        self.goals_against += goals_against;

        if goals_for > goals_against {
            self.wins += 1;
            self.points += 2; // Original game uses 2 points for a win
        } else if goals_for == goals_against {
            self.draws += 1;
            self.points += 1;
        } else {
            self.losses += 1;
        }
    }
}

// Team names from original game (lines 860-930)
pub const SERIE_A_TEAMS: [&str; 16] = [
    "ASCOLI", "ATALANTA", "AVELLINO", "COMO",
    "CREMONESE", "FIORENTINA", "INTER", "JUVENTUS",
    "LAZIO", "MILAN", "NAPOLI", "ROMA",
    "SAMPDORIA", "TORINO", "UDINESE", "VERONA",
];

pub const SERIE_B_TEAMS: [&str; 16] = [
    "AREZZO", "TRENTO", "CAGLIARI", "CATANIA",
    "CESENA", "EMPOLI", "GENOA", "LECCE",
    "MONZA", "PADOVA", "PERUGIA", "CATANZARO",
    "PISA", "TARANTO", "TRIESTINA", "VARESE",
];

pub const SERIE_C1_TEAMS: [&str; 16] = [
    "BOLOGNA", "CAMPOBASSO", "PARMA", "PIACENZA",
    "VICENZA", "RIMINI", "PISTOIESE", "BRESCIA",
    "MODENA", "SANREMESE", "SPAL", "CARRARESE",
    "ANCONA", "LIVORNO", "FOGGIA", "PALERMO",
];

pub const SERIE_C2_TEAMS: [&str; 16] = [
    "PESCARA", "MESSINA", "CAVESE", "REGGINA",
    "FRANCAVILLA", "CASARANO", "NOCERINA", "FANO",
    "MANTOVA", "SIRACUSA", "SAVONA", "SIENA",
    "VENEZIA", "ALESSANDRIA", "POTENZA", "BRINDISI",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStats {
    pub energy: u8,      // D(1) and D(8)
    pub morale: u8,      // D(2) and D(9)
    pub defense: u8,     // D(3) and D(10)
    pub midfield: u8,    // D(4) and D(11)
    pub attack: u8,      // D(5) and D(12)
    pub defense_rating: u8,  // D(6) and D(13)
    pub attack_rating: u8,   // D(7) and D(14)
}

impl TeamStats {
    pub fn new() -> Self {
        TeamStats {
            energy: 0,
            morale: 15, // K=15 initial morale
            defense: 0,
            midfield: 0,
            attack: 0,
            defense_rating: 0,
            attack_rating: 0,
        }
    }

    // Calculate derived stats using original algorithms
    pub fn calculate_ratings(&mut self) {
        // Original: D(6)=INT(D(3)+D(4)/2+D(1)/2+D(2)/2)
        self.defense_rating = ((self.defense as u16
            + self.midfield as u16 / 2
            + self.energy as u16 / 2
            + self.morale as u16 / 2) as f32) as u8;

        // Original: D(7)=INT(D(7)+(D(6))/2+(D(3))/2+(D(4))/2)
        self.attack_rating = ((self.attack as u16
            + self.defense_rating as u16 / 2
            + self.defense as u16 / 2
            + self.midfield as u16 / 2) as f32) as u8;
    }
}
