use rand::Rng;
use crate::game_state::GameState;
use crate::player::PlayerStatus;

pub struct Market {
    rng: rand::rngs::ThreadRng,
}

impl Market {
    pub fn new() -> Self {
        Market {
            rng: rand::thread_rng(),
        }
    }

    /// Try to sell a player
    /// Original: lines 1280-1460 (VENDITA section)
    pub fn sell_player(&mut self, game: &mut GameState, player_id: usize) -> Result<i32, String> {
        let player = &game.players[player_id];

        // Check if player is owned
        if matches!(player.status, PlayerStatus::NotOwned) {
            return Err(format!("You don't own {} in your squad", player.name));
        }

        // Check if there are offers (C(UZ)=1 means no offers)
        if matches!(player.status, PlayerStatus::Available) {
            return Err(format!("There are no offers for {}", player.name));
        }

        // Calculate offer price with variation
        // Original: PZ=5000*(5-N)+5000*A(UZ)
        //          PZ=INT(PZ-(RND(1)*(PZ/10))-(RND(1)*(PZ/10)))
        let base_value = player.market_value(game.current_league);
        let variation1 = self.rng.gen_range(0..(base_value / 10));
        let variation2 = self.rng.gen_range(0..(base_value / 10));
        let offer = base_value - variation1 - variation2;

        Ok(offer)
    }

    /// Complete the sale of a player
    pub fn complete_sale(&mut self, game: &mut GameState, player_id: usize, price: i32) {
        game.players[player_id].status = PlayerStatus::NotOwned;
        game.money += price;
    }

    /// Reject a sale offer
    /// Original: C(UZ)=INT(RND(1)*2)+1
    pub fn reject_sale(&mut self, game: &mut GameState, player_id: usize) {
        // Player becomes available or owned (random)
        game.players[player_id].status = if self.rng.gen_bool(0.5) {
            PlayerStatus::Available
        } else {
            PlayerStatus::Owned
        };
    }

    /// Get list of players available for purchase
    /// Original: lines 4070-4340 (ACQUISTI section)
    pub fn get_available_players(&self, game: &GameState) -> Vec<usize> {
        game.players
            .iter()
            .enumerate()
            .filter(|(_, p)| matches!(p.status, PlayerStatus::NotOwned))
            .map(|(id, _)| id)
            .collect()
    }

    /// Make an offer for a player
    /// Original: lines 4110-4340
    pub fn make_offer(
        &mut self,
        game: &mut GameState,
        player_id: usize,
        offer: i32,
    ) -> Result<bool, String> {
        let player = &game.players[player_id];

        // Check if player is available
        if !matches!(player.status, PlayerStatus::NotOwned) {
            return Err("Player is not available for purchase".to_string());
        }

        // Check if can afford
        if !game.can_afford(offer) {
            return Err(format!("You don't have {} available", offer));
        }

        // Check squad size (max 16 players)
        let owned_count = game.owned_players().len();
        if owned_count >= 16 {
            return Err("Cannot have more than 16 players in your squad".to_string());
        }

        // Calculate minimum acceptable price
        // Original: UZ=INT(XZ-(1)*(XZ/10)-(RND(1)*(XZ/10)))
        let market_value = player.market_value(game.current_league);
        let min_price = market_value
            - (market_value / 10)
            - self.rng.gen_range(0..(market_value / 10));

        if offer < min_price {
            Ok(false) // Offer rejected
        } else {
            // Offer accepted
            game.players[player_id].status = PlayerStatus::Owned;
            game.money -= offer;
            Ok(true)
        }
    }

    /// Get a random available player for purchase
    /// Original: PZ=INT(RND(1)*24)+1:IFC(PZ)<>0THEN3865
    pub fn get_random_available_player(&mut self, game: &GameState) -> Option<usize> {
        let available = self.get_available_players(game);
        if available.is_empty() {
            return None;
        }

        let idx = self.rng.gen_range(0..available.len());
        Some(available[idx])
    }

    /// List all owned players with their values
    /// Original: lines 1480-1620 (LISTA section)
    pub fn list_squad(&self, game: &GameState) -> Vec<(usize, String, PlayerStatus, i32)> {
        game.players
            .iter()
            .enumerate()
            .filter(|(_, p)| !matches!(p.status, PlayerStatus::NotOwned))
            .map(|(id, p)| {
                let value = p.market_value(game.current_league);
                (id, p.name.clone(), p.status, value)
            })
            .collect()
    }
}

/// Banking system for loans
/// Original: lines 1840-2030 (PRESTITO section)
pub fn request_loan(game: &mut GameState, amount: i32) -> Result<String, String> {
    game.take_loan(amount)?;

    Ok(format!(
        "Loan of {} approved!\n\
         Total debt: {}\n\
         Weekly interest: {}\n\
         Available: {}",
        amount, game.debt, game.weekly_interest, game.money
    ))
}

pub fn get_loan_info(game: &GameState) -> String {
    let max_debt = 100_000 * (5 - game.current_league as i32);
    let available_credit = max_debt - game.debt;

    format!(
        "SPORT BANK\n\
         Current balance: {}\n\
         Current debt: {}\n\
         Weekly interest: {}\n\
         Max debt in league: {}\n\
         Available credit: {}",
        game.money, game.debt, game.weekly_interest, max_debt, available_credit
    )
}
