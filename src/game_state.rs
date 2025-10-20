use serde::{Deserialize, Serialize};
use crate::player::{Player, PlayerStatus, PLAYER_NAMES};
use crate::team::{Team, TeamStats, SERIE_A_TEAMS, SERIE_B_TEAMS, SERIE_C1_TEAMS, SERIE_C2_TEAMS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub player_team_id: usize,  // ID of the team the player manages (index 0 is player's team)
    pub current_league: u8,     // 1=A, 2=B, 3=C1, 4=C2
    pub money: i32,             // W: starting 150000
    pub debt: i32,              // Y
    pub weekly_interest: i32,   // Z: debt/20
    pub morale: u8,             // K: 15 initially, 20 max
    pub management_level: u8,   // R: 20 initially
    pub seasons_played: u8,     // B1
    pub matches_played: u8,     // I
    pub match_results: Vec<String>, // C$(I-1) = result string
}

impl GameState {
    pub fn new(team_name: String, starting_league: u8) -> Self {
        let mut game = GameState {
            players: Vec::new(),
            teams: Vec::new(),
            player_team_id: 0,
            current_league: starting_league,
            money: 150_000,  // W=150000
            debt: 0,         // Y=0
            weekly_interest: 0, // Z=0
            morale: 15,      // K=15
            management_level: 20, // R=20
            seasons_played: 1,    // B1=1
            matches_played: 0,    // I=0
            match_results: Vec::new(),
        };

        // Initialize all 24 players from original game
        for (id, name) in PLAYER_NAMES.iter().enumerate() {
            let position = Player::position_from_id(id);
            game.players.push(Player::new(id, name.to_string(), position));
        }

        // Initialize all teams across 4 leagues (64 teams total)
        let all_teams = [
            (1, SERIE_A_TEAMS.as_slice()),
            (2, SERIE_B_TEAMS.as_slice()),
            (3, SERIE_C1_TEAMS.as_slice()),
            (4, SERIE_C2_TEAMS.as_slice()),
        ];

        let mut team_id = 0;
        for (league_level, teams) in all_teams.iter() {
            for team_name in teams.iter() {
                game.teams.push(Team::new(team_id, team_name.to_string(), *league_level));
                team_id += 1;
            }
        }

        // Set player's team
        if let Some(team) = game.teams.iter_mut().find(|t| t.name == team_name) {
            game.player_team_id = team.id;
            game.current_league = team.league_level;
        } else {
            // Default to first team in selected league
            let league_start = ((starting_league - 1) * 16) as usize;
            game.player_team_id = league_start;
            game.teams[league_start].name = team_name;
        }

        // Give player starting squad: J(1)=6, others J(x)=1
        // This represents stamina/energy for match scheduling
        game.players[0].status = PlayerStatus::Playing;
        for i in 1..8 {
            game.players[i].status = PlayerStatus::Owned;
        }

        game
    }

    pub fn player_team(&self) -> &Team {
        &self.teams[self.player_team_id]
    }

    pub fn player_team_mut(&mut self) -> &mut Team {
        &mut self.teams[self.player_team_id]
    }

    pub fn get_league_teams(&self, league: u8) -> Vec<&Team> {
        self.teams
            .iter()
            .filter(|t| t.league_level == league)
            .collect()
    }

    pub fn get_league_teams_mut(&mut self, league: u8) -> Vec<&mut Team> {
        self.teams
            .iter_mut()
            .filter(|t| t.league_level == league)
            .collect()
    }

    pub fn owned_players(&self) -> Vec<&Player> {
        self.players
            .iter()
            .filter(|p| matches!(p.status, PlayerStatus::Owned | PlayerStatus::Playing | PlayerStatus::Substitute))
            .collect()
    }

    pub fn playing_players(&self) -> Vec<&Player> {
        self.players
            .iter()
            .filter(|p| p.status == PlayerStatus::Playing)
            .collect()
    }

    pub fn can_afford(&self, amount: i32) -> bool {
        self.money >= amount
    }

    pub fn take_loan(&mut self, amount: i32) -> Result<(), String> {
        let max_debt = 100_000 * (5 - self.current_league as i32);

        if amount + self.debt > max_debt {
            return Err(format!(
                "Cannot get a loan of {}. Max debt in this league is {}",
                amount, max_debt
            ));
        }

        // Original: Y=Y+XZ*1.2:Z=Y/20:W=W+XZ
        self.debt += (amount as f32 * 1.2) as i32;
        self.weekly_interest = self.debt / 20;
        self.money += amount;

        Ok(())
    }

    pub fn pay_weekly_costs(&mut self) {
        let owned_count = self.owned_players().len();
        let field_rent = (owned_count as i32) * (70 + (5 - self.current_league as i32) * 10);
        let misc_costs = 500 * (5 - self.current_league as i32);

        let total_costs = field_rent + misc_costs + self.weekly_interest;

        self.money -= total_costs;

        // If can't afford, increase debt
        if self.money < 0 {
            let shortage = -self.money + 1000;
            self.money = 0;
            self.debt += (shortage as f32 * 1.2) as i32;
            self.weekly_interest = self.debt / 20;
        }

        // Pay down debt with interest
        self.debt -= self.weekly_interest;
        if self.debt <= 0 {
            self.debt = 0;
            self.weekly_interest = 0;
        }
    }

    pub fn calculate_player_team_stats(&self) -> TeamStats {
        let mut stats = TeamStats::new();
        stats.morale = self.morale;

        let playing = self.playing_players();
        let player_count = playing.len();

        if player_count == 0 {
            return stats;
        }

        let mut total_energy = 0u16;

        for player in &playing {
            total_energy += player.power as u16;

            // Add style to position-specific stat
            // Original: D((KJ)+3)=D((KJ)+3)+A(PZ)
            // KJ = (player_id + 1) / 8, so 0-7=0(def), 8-15=1(mid), 16-23=2(att)
            let position_index = (player.id + 1) / 8;
            match position_index {
                0 => stats.defense += player.style,
                1 => stats.midfield += player.style,
                _ => stats.attack += player.style,
            }
        }

        stats.energy = (total_energy / 11).min(20) as u8;
        stats.defense = stats.defense.min(20);
        stats.midfield = stats.midfield.min(20);
        stats.attack = stats.attack.min(20);

        stats.calculate_ratings();

        stats
    }

    pub fn get_league_standings(&self) -> Vec<(usize, &Team)> {
        let mut league_teams: Vec<_> = self.teams
            .iter()
            .enumerate()
            .filter(|(_, t)| t.league_level == self.current_league)
            .collect();

        // Sort by points (descending), then goal difference (descending)
        league_teams.sort_by(|a, b| {
            b.1.points.cmp(&a.1.points)
                .then_with(|| b.1.goal_difference().cmp(&a.1.goal_difference()))
        });

        league_teams
    }

    pub fn player_position(&self) -> usize {
        let standings = self.get_league_standings();
        standings
            .iter()
            .position(|(id, _)| *id == self.player_team_id)
            .map(|pos| pos + 1)
            .unwrap_or(16)
    }
}

pub fn league_name(level: u8) -> &'static str {
    match level {
        1 => "Serie A",
        2 => "Serie B",
        3 => "Serie C1",
        4 => "Serie C2",
        _ => "Unknown",
    }
}
