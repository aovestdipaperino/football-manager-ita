use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::game_state::{GameState, league_name};
use crate::player::PlayerStatus;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    MainMenu,
    TeamSelection,
    GameMenu,
    ViewSquad,
    ViewStandings,
    ViewFinances,
    Market,
    Banking,
    PlayMatch,
    MatchResult,
    SeasonEnd,
}

pub struct UI {
    pub current_screen: Screen,
    pub selected_index: usize,
    pub message: Option<String>,
    pub input_buffer: String,
}

impl UI {
    pub fn new() -> Self {
        UI {
            current_screen: Screen::MainMenu,
            selected_index: 0,
            message: None,
            input_buffer: String::new(),
        }
    }

    pub fn draw(&mut self, f: &mut Frame, game: &GameState) {
        match self.current_screen {
            Screen::MainMenu => self.draw_main_menu(f),
            Screen::TeamSelection => self.draw_team_selection(f, game),
            Screen::GameMenu => self.draw_game_menu(f, game),
            Screen::ViewSquad => self.draw_squad(f, game),
            Screen::ViewStandings => self.draw_standings(f, game),
            Screen::ViewFinances => self.draw_finances(f, game),
            Screen::Market => self.draw_market(f, game),
            Screen::Banking => self.draw_banking(f, game),
            _ => {}
        }
    }

    fn draw_main_menu(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let title = Paragraph::new("⚽ FOOTBALL MANAGER ⚽")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let items = vec![
            ListItem::new("1. New Game"),
            ListItem::new("2. Load Game"),
            ListItem::new("3. Quit"),
        ];

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("Use arrow keys to navigate, Enter to select")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_team_selection(&self, f: &mut Frame, game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(5),
            ])
            .split(f.area());

        let title = Paragraph::new("Select Your Team")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Show all 64 teams across all leagues
        let mut items = Vec::new();
        for league in 1..=4 {
            items.push(ListItem::new(format!("\n=== {} ===", league_name(league)))
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));

            let teams = game.get_league_teams(league);
            for team in teams {
                items.push(ListItem::new(format!("  {}", team.name)));
            }
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Teams"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[1]);

        let help = Paragraph::new(
            "↑↓: Navigate | Enter: Select team | ESC: Back\n\
             Choose a team from any division"
        )
        .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_game_menu(&self, f: &mut Frame, game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        // Header with team info
        let header_text = format!(
            "Team: {} | League: {} | Position: {}\nMoney: {} | Debt: {} | Morale: {}",
            game.player_team().name,
            league_name(game.current_league),
            game.player_position(),
            game.money,
            game.debt,
            game.morale
        );
        let header = Paragraph::new(header_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(header, chunks[0]);

        // Menu items (matching original game menu)
        let items = vec![
            ListItem::new("1. Sell or List Players"),
            ListItem::new("2. Get a Loan"),
            ListItem::new("3. View Standings"),
            ListItem::new("4. View Team Status"),
            ListItem::new("5. Change Team Names"),
            ListItem::new("6. Change Player Names"),
            ListItem::new("G. Play Match"),
            ListItem::new("R. Restart with Different Team"),
            ListItem::new("Q. Quit"),
        ];

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Main Menu"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_widget(list, chunks[1]);

        let help = Paragraph::new("Press the number/letter of your choice")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_squad(&self, f: &mut Frame, game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(4),
            ])
            .split(f.area());

        let title = Paragraph::new(format!("Squad: {}", game.player_team().name))
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let mut items = Vec::new();
        items.push(ListItem::new(format!(
            "{:<3} {:<15} {:<8} {:<6} {:<6} {:<10} {:<8}",
            "ID", "Name", "Pos", "Style", "Power", "Value", "Status"
        ))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));

        for (idx, player) in game.players.iter().enumerate() {
            if !matches!(player.status, PlayerStatus::NotOwned) {
                let pos = match player.position {
                    crate::player::PlayerPosition::Defender => "DEF",
                    crate::player::PlayerPosition::Midfielder => "MID",
                    crate::player::PlayerPosition::Forward => "FWD",
                };

                let status = match player.status {
                    PlayerStatus::Playing => "Playing",
                    PlayerStatus::Substitute => "Sub",
                    PlayerStatus::Owned => "Owned",
                    _ => "Available",
                };

                let value = player.market_value(game.current_league);

                let item_text = format!(
                    "{:<3} {:<15} {:<8} {:<6} {:<6} {:<10} {:<8}",
                    idx, player.name, pos, player.style, player.power, value, status
                );

                items.push(ListItem::new(item_text));
            }
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Your Players"));

        f.render_widget(list, chunks[1]);

        let help = Paragraph::new(
            "* = Injured | S = Substitute | G = Playing\n\
             ESC: Back to menu"
        )
        .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_standings(&self, f: &mut Frame, game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let title = Paragraph::new(format!(
            "League Standings: {} - After {} matches",
            league_name(game.current_league),
            game.matches_played
        ))
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let mut items = Vec::new();
        items.push(ListItem::new(format!(
            "{:<3} {:<20} {:<4} {:<4} {:<6} {:<5}",
            "Pos", "Team", "GF", "GA", "Pts", "GD"
        ))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));

        let standings = game.get_league_standings();
        for (pos, (team_id, team)) in standings.iter().enumerate() {
            let is_player = *team_id == game.player_team_id;
            let style = if is_player {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let item_text = format!(
                "{:<3} {:<20} {:<4} {:<4} {:<6} {:+5}",
                pos + 1,
                team.name,
                team.goals_for,
                team.goals_against,
                team.points,
                team.goal_difference()
            );

            items.push(ListItem::new(item_text).style(style));
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Table"));

        f.render_widget(list, chunks[1]);

        let help = Paragraph::new(format!("Your position: {}  |  ESC: Back", game.player_position()))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_finances(&self, f: &mut Frame, game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(f.area());

        let title = Paragraph::new("Team Finances")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let max_debt = 100_000 * (5 - game.current_league as i32);
        let management_rating = if game.seasons_played == 1 {
            20
        } else {
            game.management_level.saturating_sub(game.seasons_played - 1)
        };

        let info_text = format!(
            "Team: {}\n\
             League: {}\n\
             \n\
             Balance: {}\n\
             Debt: {}\n\
             Weekly Interest: {}\n\
             Max Debt (this league): {}\n\
             \n\
             Management Level: {}\n\
             Seasons Played: {}\n\
             Team Morale: {}\n\
             Position: {}\n\
             \n\
             Owned Players: {}",
            game.player_team().name,
            league_name(game.current_league),
            game.money,
            game.debt,
            game.weekly_interest,
            max_debt,
            management_rating,
            game.seasons_played,
            game.morale,
            game.player_position(),
            game.owned_players().len()
        );

        let paragraph = Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);

        let help = Paragraph::new("ESC: Back to menu")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_market(&self, f: &mut Frame, _game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(4),
            ])
            .split(f.area());

        let title = Paragraph::new("Transfer Market")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let help = Paragraph::new(
            "Buy and sell players here\n\
             ESC: Back to menu"
        )
        .style(Style::default().fg(Color::DarkGray));
        f.render_widget(help, chunks[2]);
    }

    fn draw_banking(&self, f: &mut Frame, game: &GameState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(4),
            ])
            .split(f.area());

        let title = Paragraph::new("SPORT BANK")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let max_debt = 100_000 * (5 - game.current_league as i32);
        let available = max_debt - game.debt;

        let info = format!(
            "Current Balance: {}\n\
             Current Debt: {}\n\
             Weekly Interest: {} (paid automatically)\n\
             \n\
             Maximum Debt in {}: {}\n\
             Available Credit: {}\n\
             \n\
             Enter loan amount (ESC to cancel)",
            game.money,
            game.debt,
            game.weekly_interest,
            league_name(game.current_league),
            max_debt,
            available
        );

        let paragraph = Paragraph::new(info)
            .block(Block::default().borders(Borders::ALL).title("Loan Information"))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);

        if let Some(ref msg) = self.message {
            let msg_widget = Paragraph::new(msg.as_str())
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: true });
            f.render_widget(msg_widget, chunks[2]);
        }
    }
}
