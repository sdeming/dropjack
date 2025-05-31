// Sub-modules
pub mod board;
pub mod difficulty;
pub mod effects;
pub mod game_state;
pub mod playing_card;
pub mod position;

use self::board::Board;
use self::effects::DelayedDestruction;
use self::playing_card::PlayingCard;
use self::position::{Position, VisualPosition};
use crate::cards::{Card, Deck};
use crate::database::{Database, HighScore};
use std::path::Path;
use std::time::{Duration, Instant};

// Re-export for backward compatibility
pub use self::difficulty::Difficulty;
pub use self::game_state::{GameOver, GameState, Paused, Playing, QuitConfirm, StartScreen};

const COMBINATION_DELAY: u64 = 300;

// Main game struct
pub struct Game {
    pub state: Box<dyn GameState>,
    pub board: Board,
    pub deck: Deck,
    pub current_card: Option<PlayingCard>,
    pub next_card: Option<Card>,
    pub score: i32,
    pub difficulty: Difficulty,
    pub fall_speed: Duration,
    pub last_fall_time: Instant,
    pub speed_increase_interval: Duration,
    pub last_speed_increase: Instant,
    pub database: Database,
    pub high_scores: Vec<HighScore>,
    pub player_initials: String,
    pub pending_explosions: Vec<(i32, i32, Card)>,
    pub delayed_destructions: Vec<DelayedDestruction>,
    pub last_dropped_x: Option<i32>,
}

impl Game {
    pub fn new(db_path: &Path) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let board = Board::new(10, 15, 48);

        let database = Database::new(db_path).expect("Failed to create database");
        let high_scores = database.get_high_scores(10).unwrap_or_default();

        let next_card = deck.draw();

        Game {
            state: Box::new(StartScreen),
            board,
            deck,
            current_card: None,
            next_card,
            score: 0,
            difficulty: Difficulty::Easy,
            fall_speed: Duration::from_millis(1000), // Initial fall speed: 1 second
            last_fall_time: Instant::now(),
            speed_increase_interval: Duration::from_secs(30), // Increase speed every 30 seconds
            last_speed_increase: Instant::now(),
            database,
            high_scores,
            player_initials: String::new(),
            pending_explosions: Vec::new(),
            delayed_destructions: Vec::new(),
            last_dropped_x: None, // Initialize to None for the first card
        }
    }

    pub fn start_game(&mut self, difficulty: Difficulty) {
        self.state = Box::new(Playing);
        self.difficulty = difficulty;
        self.score = 0;
        self.fall_speed = Duration::from_millis(1000);
        self.last_fall_time = Instant::now();
        self.last_speed_increase = Instant::now();
        self.player_initials = String::new();
        self.last_dropped_x = None;

        // Reset the board
        self.board = Board::new(self.board.width, self.board.height, 48);

        // Reset the deck
        self.deck.reset();

        // Draw the first card
        self.spawn_new_card();
    }

    pub fn spawn_new_card(&mut self) {
        if let Some(card) = self.next_card {
            let x = self.last_dropped_x.unwrap_or(self.board.width / 2);
            let position = Position { x, y: 0 };
            self.current_card = Some(PlayingCard {
                card,
                position,
                visual_position: VisualPosition {
                    x: (x * self.board.cell_size) as f32,
                    y: 0f32, // (0 * self.board.cell_size) as f32
                },
                target: Position { x, y: 0 },
                is_falling: false,
            });
            self.next_card = self.deck.draw();

            if self.next_card.is_none() {
                self.deck.reset();
                self.next_card = self.deck.draw();
            }
        }
    }

    pub fn update(&mut self) {
        if self.state.should_update() {
            self.update_playing_state();
        }
    }

    pub fn update_playing_state(&mut self) {
        self.process_card_removals();
        self.process_delayed_destructions();
        self.update_animations();
        self.handle_card_spawning();
        self.handle_auto_speed_increase();
        self.handle_automatic_card_fall();
        self.check_game_over();
    }

    fn process_card_removals(&mut self) {
        let removed_cards = self.board.process_marked_removals();
        if !removed_cards.is_empty() {
            for (x, y, card) in removed_cards {
                self.pending_explosions.push((x, y, card));

                // Calculate and add the score
                let base_score = 21;
                self.score += base_score;
            }

            // Apply gravity after removals
            while self.board.apply_gravity() {}
        }
    }

    fn update_animations(&mut self) {
        // Update falling card animations
        self.board.update_falling_cards();

        // Update current card position animation
        if let Some(ref mut playing_card) = self.current_card {
            let move_speed = 12.0; // pixels per frame - scaled up for larger cells

            // Horizontal movement
            let target_x = (playing_card.target.x * self.board.cell_size) as f32;
            if playing_card.visual_position.x != target_x {
                let diff_x = target_x - playing_card.visual_position.x;
                let move_x = if diff_x.abs() <= move_speed {
                    diff_x
                } else {
                    move_speed * diff_x.signum()
                };
                playing_card.visual_position.x += move_x;

                // Snap to target when close enough
                if (playing_card.visual_position.x - target_x).abs() < 0.1 {
                    playing_card.visual_position.x = target_x;
                    playing_card.position.x = playing_card.target.x;
                }
            }

            // Vertical movement (falling)
            let target_y = (playing_card.target.y * self.board.cell_size) as f32;
            if playing_card.is_falling && playing_card.visual_position.y != target_y {
                let fall_speed = 8.0;
                let diff_y = target_y - playing_card.visual_position.y;
                let move_y = if diff_y.abs() <= fall_speed {
                    diff_y
                } else {
                    fall_speed * diff_y.signum()
                };
                playing_card.visual_position.y += move_y;

                if (playing_card.visual_position.y - target_y).abs() < 0.1 {
                    playing_card.visual_position.y = target_y;
                    playing_card.position.y = playing_card.target.y;
                    playing_card.is_falling = false;
                }
            }
        }
    }

    fn handle_card_spawning(&mut self) {
        // Only process card spawning if not processing combinations
        if self.current_card.is_none() {
            // No current card - spawn a new one
            self.spawn_new_card();
        }
    }

    fn handle_auto_speed_increase(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_speed_increase) >= self.speed_increase_interval {
            self.increase_speed();
            self.last_speed_increase = now;
        }
    }

    fn handle_automatic_card_fall(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_fall_time) >= self.fall_speed {
            self.move_current_card_down();
            self.last_fall_time = now;
        }
    }

    fn check_game_over(&mut self) {
        if self.board.is_game_over() {
            self.transition_to_game_over();
        }
    }

    fn increase_speed(&mut self) {
        // Decrease fall time by 10% (increase speed)
        let new_fall_time = self.fall_speed.as_millis() * 9 / 10;
        self.fall_speed = Duration::from_millis(new_fall_time as u64);
    }

    pub fn move_current_card_left(&mut self) {
        if let Some(ref mut playing_card) = self.current_card {
            // Only allow movement if not currently moving horizontally
            if playing_card.position.x == playing_card.target.x {
                let new_x = playing_card.position.x - 1;
                if new_x >= 0 && self.board.is_cell_empty(new_x, playing_card.position.y) {
                    playing_card.target.x = new_x;
                }
            }
        }
    }

    pub fn move_current_card_right(&mut self) {
        if let Some(ref mut playing_card) = self.current_card {
            // Only allow movement if not currently moving horizontally
            if playing_card.position.x == playing_card.target.x {
                let new_x = playing_card.position.x + 1;
                if new_x < self.board.width
                    && self.board.is_cell_empty(new_x, playing_card.position.y)
                {
                    playing_card.target.x = new_x;
                }
            }
        }
    }

    pub fn move_current_card_down(&mut self) {
        if let Some(ref mut playing_card) = self.current_card {
            let new_y = playing_card.position.y + 1;

            if new_y < self.board.height && self.board.is_cell_empty(playing_card.position.x, new_y)
            {
                if !playing_card.is_falling {
                    playing_card.target.y = new_y;
                    playing_card.is_falling = true;
                }
            } else if !playing_card.is_falling {
                playing_card.visual_position.x =
                    (playing_card.position.x * self.board.cell_size) as f32;
                playing_card.target.x = playing_card.position.x;
                self.place_current_card();
                self.process_combinations();
                self.spawn_new_card();
            }
        }
    }

    pub fn hard_drop(&mut self) {
        if let Some(ref mut playing_card) = self.current_card {
            // Find the lowest position the card can go
            let mut new_y = playing_card.position.y;
            while new_y + 1 < self.board.height
                && self.board.is_cell_empty(playing_card.position.x, new_y + 1)
            {
                new_y += 1;
            }

            // Update both logical and visual positions immediately (instant drop)
            playing_card.position.y = new_y;
            playing_card.visual_position.y = (new_y * self.board.cell_size) as f32;
            playing_card.target.y = new_y;
            playing_card.is_falling = false;

            // Ensure horizontal position is also synced
            playing_card.visual_position.x =
                (playing_card.position.x * self.board.cell_size) as f32;
            playing_card.target.x = playing_card.position.x;

            self.place_current_card();

            // Check for combinations and apply cascading effects
            self.process_combinations();

            // Spawn new card (current_card is now None after place_current_card)
            self.spawn_new_card();
        }
    }

    fn place_current_card(&mut self) {
        if let Some(playing_card) = self.current_card.take() {
            // Store the X position of this dropped card for the next card
            self.last_dropped_x = Some(playing_card.position.x);
            self.board.place_card(
                playing_card.position.x,
                playing_card.position.y,
                playing_card.card,
            );
        }
    }

    pub fn save_high_score(&mut self) {
        use chrono::Local;

        let high_score = HighScore {
            id: None,
            player_initials: self.player_initials.clone(),
            score: self.score,
            difficulty: self.difficulty.to_string(),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };

        if self.database.add_high_score(&high_score).is_ok() {
            // Refresh high scores
            if let Ok(scores) = self.database.get_high_scores(10) {
                self.high_scores = scores;
            }
        }
    }

    pub fn add_initial(&mut self, c: char) {
        if self.player_initials.len() < 3 && c.is_ascii_alphabetic() {
            self.player_initials.push(c.to_ascii_uppercase());
        }
    }

    pub fn remove_initial(&mut self) {
        self.player_initials.pop();
    }

    // Process combinations with delayed cascading effect for better visual appeal
    fn process_combinations(&mut self) {
        // Find all combinations
        let all_combinations = self.board.check_combinations(self.difficulty);
        if all_combinations.is_empty() {
            return; // No combinations found
        }

        // Clear any existing delayed destructions
        self.delayed_destructions.clear();

        // Process each card individually with staggered timing
        let now = Instant::now();
        let delay_between_cards = Duration::from_millis(COMBINATION_DELAY);

        for (card_index, &position) in all_combinations.iter().enumerate() {
            let removal_time = now + delay_between_cards * card_index as u32;

            // Mark this individual card for delayed removal
            self.board
                .mark_cards_for_removal(vec![position], removal_time);
        }

        // Schedule a check for new combinations after all cards are processed
        let final_check_time = now + delay_between_cards * all_combinations.len() as u32;
        self.delayed_destructions.push(DelayedDestruction {
            destruction_time: final_check_time,
            chain_multiplier: 2,
            combination_index: 1,
        });
    }

    // Get and clear pending explosions
    pub fn take_pending_explosions(&mut self) -> Vec<(i32, i32, Card)> {
        std::mem::take(&mut self.pending_explosions)
    }

    // Process delayed destructions
    fn process_delayed_destructions(&mut self) {
        let now = Instant::now();
        let mut processed_any = false;
        let mut new_destructions = Vec::new();
        let mut cascade_checks = Vec::new();

        // First pass: collect what needs to be done
        self.delayed_destructions.retain(|destruction| {
            if now >= destruction.destruction_time {
                cascade_checks.push((destruction.chain_multiplier, destruction.combination_index));
                processed_any = true;
                false // Remove this destruction from the queue
            } else {
                true // Keep this destruction in the queue
            }
        });

        // Second pass: handle the cascade checks
        for (chain_multiplier, combination_index) in cascade_checks {
            let new_combinations = self.board.check_combinations(self.difficulty);

            if !new_combinations.is_empty() {
                // Found new combinations! Mark them for delayed removal
                let delay_between_cards = Duration::from_millis(COMBINATION_DELAY);

                for (card_index, &position) in new_combinations.iter().enumerate() {
                    let removal_time = now + delay_between_cards * card_index as u32;
                    self.board
                        .mark_cards_for_removal(vec![position], removal_time);
                }

                // Schedule next cascade check
                let final_check_time = now + delay_between_cards * new_combinations.len() as u32;
                new_destructions.push(DelayedDestruction {
                    destruction_time: final_check_time,
                    chain_multiplier: chain_multiplier + 1,
                    combination_index: combination_index + 1,
                });

                // Add cascade bonus
                let cascade_bonus = 50;
                self.score += cascade_bonus;
            } else {
                // No more combinations found - end the cascade
            }
        }

        // Add any new destructions that were scheduled
        self.delayed_destructions.extend(new_destructions);

        // If we processed and no more destructions are queued, we're done
        if processed_any && self.delayed_destructions.is_empty() {}
    }

    // Helper methods for state management
    pub fn is_playing(&self) -> bool {
        self.state.state_name() == "Playing"
    }

    pub fn is_paused(&self) -> bool {
        self.state.state_name() == "Paused"
    }

    pub fn is_start_screen(&self) -> bool {
        self.state.state_name() == "StartScreen"
    }

    pub fn is_game_over(&self) -> bool {
        self.state.state_name() == "GameOver"
    }

    pub fn is_quit_confirm(&self) -> bool {
        self.state.state_name() == "QuitConfirm"
    }

    pub fn transition_to_start_screen(&mut self) {
        self.state = Box::new(StartScreen);
    }

    pub fn transition_to_playing(&mut self) {
        self.state = Box::new(Playing);
    }

    pub fn transition_to_paused(&mut self) {
        self.state = Box::new(Paused);
    }

    pub fn transition_to_game_over(&mut self) {
        self.state = Box::new(GameOver);
    }

    pub fn transition_to_quit_confirm(&mut self) {
        self.state = Box::new(QuitConfirm);
    }
}
