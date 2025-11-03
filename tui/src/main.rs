mod game_state;
mod market;
mod match_engine;
mod player;
mod team;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use std::time::Duration;

use crate::game_state::GameState;
use crate::market::Market;
use crate::match_engine::MatchEngine;
use crate::ui::{Screen, UI};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut ui = UI::new();
    let mut game: Option<GameState> = None;
    let mut market = Market::new();
    let mut match_engine = MatchEngine::new();

    // Run app
    let res = run_app(&mut terminal, &mut ui, &mut game, &mut market, &mut match_engine);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    ui: &mut UI,
    game: &mut Option<GameState>,
    market: &mut Market,
    match_engine: &mut MatchEngine,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            if let Some(ref game_state) = game {
                ui.draw(f, game_state);
            } else {
                ui.draw(f, &GameState::new("PLACEHOLDER".to_string(), 1));
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match ui.current_screen {
                    Screen::MainMenu => {
                        match key.code {
                            KeyCode::Char('1') => {
                                // New game - initialize with default team
                                *game = Some(GameState::new("YOUR TEAM".to_string(), 1));
                                ui.current_screen = Screen::GameMenu;
                            }
                            KeyCode::Char('3') | KeyCode::Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    Screen::GameMenu => {
                        if let Some(ref mut game_state) = game {
                            match key.code {
                                KeyCode::Char('1') => {
                                    ui.current_screen = Screen::ViewSquad;
                                }
                                KeyCode::Char('2') => {
                                    ui.current_screen = Screen::Banking;
                                }
                                KeyCode::Char('3') => {
                                    ui.current_screen = Screen::ViewStandings;
                                }
                                KeyCode::Char('4') => {
                                    ui.current_screen = Screen::ViewFinances;
                                }
                                KeyCode::Char('g') | KeyCode::Char('G') => {
                                    // Play match
                                    play_match(game_state, match_engine);
                                    ui.message = Some("Match completed! Check standings.".to_string());
                                }
                                KeyCode::Char('r') | KeyCode::Char('R') => {
                                    *game = None;
                                    ui.current_screen = Screen::MainMenu;
                                }
                                KeyCode::Char('q') | KeyCode::Char('Q') => {
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                    }
                    Screen::ViewSquad
                    | Screen::ViewStandings
                    | Screen::ViewFinances
                    | Screen::Market => {
                        if key.code == KeyCode::Esc {
                            ui.current_screen = Screen::GameMenu;
                            ui.message = None;
                        }
                    }
                    Screen::Banking => {
                        if let Some(ref mut game_state) = game {
                            match key.code {
                                KeyCode::Esc => {
                                    ui.current_screen = Screen::GameMenu;
                                    ui.message = None;
                                    ui.input_buffer.clear();
                                }
                                KeyCode::Char(c) if c.is_ascii_digit() => {
                                    ui.input_buffer.push(c);
                                }
                                KeyCode::Backspace => {
                                    ui.input_buffer.pop();
                                }
                                KeyCode::Enter => {
                                    if let Ok(amount) = ui.input_buffer.parse::<i32>() {
                                        match market::request_loan(game_state, amount) {
                                            Ok(msg) => {
                                                ui.message = Some(msg);
                                            }
                                            Err(err) => {
                                                ui.message = Some(format!("Error: {}", err));
                                            }
                                        }
                                    }
                                    ui.input_buffer.clear();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn play_match(game: &mut GameState, match_engine: &mut MatchEngine) {
    // Pick a random opponent from the same league
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let league_teams: Vec<_> = game
        .teams
        .iter()
        .enumerate()
        .filter(|(id, t)| t.league_level == game.current_league && *id != game.player_team_id)
        .map(|(id, _)| id)
        .collect();

    if league_teams.is_empty() {
        return;
    }

    let opponent_id = league_teams[rng.gen_range(0..league_teams.len())];
    let is_home = rng.gen_bool(0.5);

    // Simulate the match
    let result = match_engine.simulate_match(game, opponent_id, is_home);

    // Update game state
    let (player_goals, opponent_goals) = if is_home {
        (result.home_goals, result.away_goals)
    } else {
        (result.away_goals, result.home_goals)
    };

    game.player_team_mut()
        .record_result(player_goals, opponent_goals);
    game.teams[opponent_id].record_result(opponent_goals, player_goals);

    // Update morale
    MatchEngine::update_morale(game, player_goals, opponent_goals);

    // Add match income
    game.money += result.attendance_income;

    // Simulate other matches
    match_engine.simulate_other_matches(game);

    // Increment matches played
    game.matches_played += 1;

    // Pay weekly costs
    game.pay_weekly_costs();

    // Update player powers
    for player in &mut game.players {
        player.update_power();
    }
}
