use rand::Rng;
use crate::game_state::GameState;
use crate::team::{Team, TeamStats};

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub home_team: String,
    pub away_team: String,
    pub home_goals: u16,
    pub away_goals: u16,
    pub is_player_home: bool,
    pub attendance_income: i32,
}

pub struct MatchEngine {
    rng: rand::rngs::ThreadRng,
}

impl MatchEngine {
    pub fn new() -> Self {
        MatchEngine {
            rng: rand::thread_rng(),
        }
    }

    /// Simulate a match using the original C64 algorithms
    pub fn simulate_match(
        &mut self,
        game: &GameState,
        opponent_id: usize,
        is_home: bool,
    ) -> MatchResult {
        let player_team = game.player_team();
        let opponent = &game.teams[opponent_id];

        // Calculate team stats using original algorithms
        let player_stats = game.calculate_player_team_stats();
        let opponent_stats = self.calculate_opponent_stats(game, opponent);

        // Simulate the match
        let (home_goals, away_goals) = if is_home {
            self.simulate_half(&player_stats, &opponent_stats, game.matches_played)
        } else {
            let (away, home) = self.simulate_half(&player_stats, &opponent_stats, game.matches_played);
            (home, away)
        };

        // Calculate attendance income
        // Original: A2=(17-V(1))*INT(RND(1)*400)+500*(5-N)
        let position = game.player_position();
        let attendance_income = (17 - position as i32)
            * self.rng.gen_range(0..400)
            + 500 * (5 - game.current_league as i32);

        MatchResult {
            home_team: if is_home {
                player_team.name.clone()
            } else {
                opponent.name.clone()
            },
            away_team: if is_home {
                opponent.name.clone()
            } else {
                player_team.name.clone()
            },
            home_goals,
            away_goals,
            is_player_home: is_home,
            attendance_income,
        }
    }

    /// Calculate opponent team statistics using original algorithm
    /// Original code: lines 3360-3430
    fn calculate_opponent_stats(&mut self, game: &GameState, opponent: &Team) -> TeamStats {
        let mut stats = TeamStats::new();

        // Lines 3360-3400: Calculate opponent stats based on points and matches
        for stat_index in 0..5 {
            let base_stat = if game.matches_played == 0 {
                if opponent.points == 0 {
                    self.rng.gen_range(6..=15)
                } else {
                    self.rng.gen_range(10..=19)
                }
            } else if opponent.points == 0 && game.matches_played > 0 {
                self.rng.gen_range(10..=19)
            } else {
                self.rng.gen_range(0..((opponent.points + game.matches_played as u16 * 3) as u8 + 10))
            };

            let stat = base_stat.min(20);

            match stat_index {
                0 => stats.energy = stat,
                1 => stats.morale = stat,
                2 => stats.defense = stat,
                3 => stats.midfield = stat,
                4 => stats.attack = stat,
                _ => {}
            }
        }

        // Original: D(13)=INT(D(10)+D(11)/2+D(8)/2+D(9)/2)
        stats.defense_rating = ((stats.defense as u16
            + stats.midfield as u16 / 2
            + stats.energy as u16 / 2
            + stats.morale as u16 / 2) as f32) as u8;

        // Original: D(14)=INT(D(12)+(D(11))/2+(D(8))/2+(D(9))/2)
        stats.attack_rating = ((stats.attack as u16
            + stats.midfield as u16 / 2
            + stats.energy as u16 / 2
            + stats.morale as u16 / 2) as f32) as u8;

        stats.calculate_ratings();

        stats
    }

    /// Simulate a full match (both halves)
    /// Original: lines 3660-3730
    fn simulate_half(&mut self, home_stats: &TeamStats, away_stats: &TeamStats, _matches_played: u8) -> (u16, u16) {
        let mut home_goals = 0u16;
        let mut away_goals = 0u16;

        // HZ is the combined rating used for chance frequency
        // Original: HZ=D(7)+D(13)+D(14)+D(6)
        let hz = (home_stats.attack_rating as u16
            + home_stats.defense_rating as u16
            + away_stats.attack_rating as u16
            + away_stats.defense_rating as u16)
            .max(15);

        // Simulate first half
        for _ in 0..5 {
            // Original: IFINT(RND(1)*HZ)+1<=9THENGOSUB20270
            if self.rng.gen_range(1..=hz) <= 9 {
                // Chance for a goal
                if self.rng.gen_range(0..2) == 0 {
                    // Home team chance
                    // Original: IFINT(RND(1)*D(7)-(RND(1)*D(13)))>0
                    if self.rng.gen_range(0..home_stats.attack_rating as i16)
                        - self.rng.gen_range(0..away_stats.defense_rating as i16)
                        > 0
                    {
                        home_goals += 1;
                    }
                } else {
                    // Away team chance
                    // Original: IFINT(RND(1)*D(14)-(RND(1)*D(6)))>0
                    if self.rng.gen_range(0..away_stats.attack_rating as i16)
                        - self.rng.gen_range(0..home_stats.defense_rating as i16)
                        > 0
                    {
                        away_goals += 1;
                    }
                }
            }
        }

        // Simulate second half (same algorithm)
        for _ in 0..5 {
            if self.rng.gen_range(1..=hz) <= 10 {
                if self.rng.gen_range(0..2) == 0 {
                    if self.rng.gen_range(0..home_stats.attack_rating as i16)
                        - self.rng.gen_range(0..away_stats.defense_rating as i16)
                        > 0
                    {
                        home_goals += 1;
                    }
                } else {
                    if self.rng.gen_range(0..away_stats.attack_rating as i16)
                        - self.rng.gen_range(0..home_stats.defense_rating as i16)
                        > 0
                    {
                        away_goals += 1;
                    }
                }
            }
        }

        (home_goals, away_goals)
    }

    /// Simulate matches for other teams in the league
    /// Original: lines 20640-20820
    pub fn simulate_other_matches(&mut self, game: &mut GameState) -> Vec<(String, String, u16, u16)> {
        let mut results = Vec::new();

        // Collect team data (id, name, points) to avoid borrow issues
        let league_teams: Vec<_> = game
            .teams
            .iter()
            .enumerate()
            .filter(|(_, t)| t.league_level == game.current_league)
            .map(|(id, t)| (id, t.name.clone(), t.points))
            .collect();

        let mut matched = vec![false; 16];
        matched[game.player_position() - 1] = true; // Player already played

        // Create 7 matches for remaining 14 teams
        for _ in 0..7 {
            // Find two unmatched teams
            let home_idx = loop {
                let idx = self.rng.gen_range(0..16);
                if !matched[idx] {
                    matched[idx] = true;
                    break idx;
                }
            };

            let away_idx = loop {
                let idx = self.rng.gen_range(0..16);
                if !matched[idx] {
                    matched[idx] = true;
                    break idx;
                }
            };

            let (home_id, home_name, home_points) = &league_teams[home_idx];
            let (away_id, away_name, away_points) = &league_teams[away_idx];

            // Simulate based on team points
            let home_goals = self.calculate_team_goals(*home_points, game.matches_played);
            let away_goals = self.calculate_team_goals(*away_points, game.matches_played);

            // Update team records
            game.teams[*home_id].record_result(home_goals, away_goals);
            game.teams[*away_id].record_result(away_goals, home_goals);

            results.push((
                home_name.clone(),
                away_name.clone(),
                home_goals,
                away_goals,
            ));
        }

        results
    }

    /// Calculate goals for AI teams
    /// Original: lines 20710-20760
    fn calculate_team_goals(&mut self, points: u16, matches_played: u8) -> u16 {
        let base = if points as f32 + matches_played as f32 > 1.5 {
            self.rng.gen_range(0..2 * matches_played as u16)
        } else {
            points
        };

        self.rng.gen_range(0..=(base + matches_played as u16 * 3))
    }

    /// Update morale after match result
    /// Original: lines 20520-20570
    pub fn update_morale(game: &mut GameState, player_goals: u16, opponent_goals: u16) {
        if player_goals > opponent_goals {
            // Win: K=INT(20-K)/2+K
            game.morale = ((20 - game.morale) / 2 + game.morale).min(20);
        } else if player_goals == opponent_goals {
            // Draw: K=INT(K/2)+1
            game.morale = (game.morale / 2 + 1).max(1);
        } else {
            // Loss: morale decreases
            game.morale = game.morale.saturating_sub(2).max(1);
        }
    }
}
