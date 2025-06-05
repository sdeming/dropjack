// Sub-modules
pub mod board;
pub mod states;

use self::board::Board;
use crate::database::Database;
use crate::models::{
    Card, Deck, DelayedDestruction, Difficulty, HighScore, PlayingCard, Position, VisualPosition,
};
use std::path::Path;
use std::time::{Duration, Instant};

pub use self::states::{GameOver, GameState, Paused, Playing, QuitConfirm, StartScreen};

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
    pub pending_audio_events: Vec<AudioEvent>,
    pub hard_dropping_cards: Vec<PlayingCard>, // Cards that are hard dropping and still animating
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AudioEvent {
    DifficultyChange,
    StartGame,
    DropCard,
    MakeMatch,
    ExplodeCard,
    PauseGame,
    ResumeGame,
    ForfeitGame,
    GameOver,
    OpenQuitConfirmation,
    ReturnToGame,
    QuitGame,
    // Card movement events
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
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
            pending_audio_events: Vec::new(),
            hard_dropping_cards: Vec::new(),
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
        self.hard_dropping_cards.clear();

        // Reset the board
        self.board = Board::new(self.board.width, self.board.height, 48);

        // Reset the deck
        self.deck.reset();

        // Draw the first card
        self.spawn_new_card();

        // Add audio event for starting game
        self.add_audio_event(AudioEvent::StartGame);
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
                is_hard_dropping: false,
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
            // Add audio event for making match
            self.add_audio_event(AudioEvent::MakeMatch);

            for (x, y, card) in removed_cards {
                self.pending_explosions.push((x, y, card));

                // Add audio event for exploding card
                self.add_audio_event(AudioEvent::ExplodeCard);

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
                // Use faster fall speed for hard drops
                let fall_speed = if playing_card.is_hard_dropping { 20.0 } else { 8.0 };
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
                    playing_card.is_hard_dropping = false;
                }
            }
        }

        // Update hard-dropping cards that are no longer the current card
        let mut cards_to_place = Vec::new();
        for (index, card) in self.hard_dropping_cards.iter_mut().enumerate() {
            let target_y = (card.target.y * self.board.cell_size) as f32;
            if card.is_falling && card.visual_position.y != target_y {
                let fall_speed = 20.0; // Fast fall speed for hard drops
                let diff_y = target_y - card.visual_position.y;
                let move_y = if diff_y.abs() <= fall_speed {
                    diff_y
                } else {
                    fall_speed * diff_y.signum()
                };
                card.visual_position.y += move_y;

                if (card.visual_position.y - target_y).abs() < 0.1 {
                    card.visual_position.y = target_y;
                    card.position.y = card.target.y;
                    card.is_falling = false;
                    card.is_hard_dropping = false;
                    cards_to_place.push(index);
                }
            }
        }

        // Place cards that have finished falling and remove them from hard_dropping_cards
        for index in cards_to_place.into_iter().rev() {
            let finished_card = self.hard_dropping_cards.remove(index);
            // Don't update last_dropped_x here - that should only be set when the player places a card normally
            self.board.place_card(
                finished_card.position.x,
                finished_card.position.y,
                finished_card.card,
            );

            // Add audio event for dropping card
            self.add_audio_event(AudioEvent::DropCard);

            // Process combinations after placing the card
            self.process_combinations();
            
            // Apply gravity to handle any floating cards after combinations
            while self.board.apply_gravity() {}
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

    /// Checks if a move to a new logical position is valid.
    /// This prevents a card from moving into or through an occupied space.
    fn is_move_valid(&self, current_x: i32, current_y: i32, new_x: i32, new_y: i32) -> bool {
        // Check the target cell
        if !self.board.is_cell_empty(new_x, new_y) {
            return false;
        }

        // Check for diagonal clipping: if moving both horizontally and vertically,
        // ensure the corner cell is also empty to prevent clipping.
        if new_x != current_x && new_y != current_y {
            if !self.board.is_cell_empty(current_x, new_y)
                || !self.board.is_cell_empty(new_x, current_y)
            {
                return false;
            }
        }

        true
    }

    pub fn move_current_card_left(&mut self) {
        if let Some(card) = &self.current_card {
            // Check only the immediate horizontal destination.
            // Let the robust `move_current_card_down` handle fall logic.
            if card.target.x == card.position.x {
                let new_x = card.position.x - 1;
                if new_x >= 0 && self.board.is_cell_empty(new_x, card.position.y) {
                    if let Some(card_mut) = self.current_card.as_mut() {
                        card_mut.target.x = new_x;
                        self.add_audio_event(AudioEvent::MoveLeft);
                    }
                }
            }
        }
    }

    pub fn move_current_card_right(&mut self) {
        if let Some(card) = &self.current_card {
            // Check only the immediate horizontal destination.
            if card.target.x == card.position.x {
                let new_x = card.position.x + 1;
                if new_x < self.board.width && self.board.is_cell_empty(new_x, card.position.y) {
                    if let Some(card_mut) = self.current_card.as_mut() {
                        card_mut.target.x = new_x;
                        self.add_audio_event(AudioEvent::MoveRight);
                    }
                }
            }
        }
    }

    pub fn move_current_card_down(&mut self) {
        if let Some(card) = self.current_card.as_ref() {
            let current_pos = card.position;
            let target_x = card.target.x;
            let next_y = current_pos.y + 1;

            if next_y >= self.board.height {
                self.place_current_card();
                return;
            }

            // First, check if the ideal target position (e.g., diagonal) is valid.
            let can_move_to_target =
                self.is_move_valid(current_pos.x, current_pos.y, target_x, next_y);

            // As a fallback, check if the card can at least fall straight down.
            let can_fall_vertically =
                self.is_move_valid(current_pos.x, current_pos.y, current_pos.x, next_y);

            if can_move_to_target {
                // Best case: move towards the player's intended target.
                if let Some(card_mut) = self.current_card.as_mut() {
                    card_mut.target.y = next_y;
                    card_mut.is_falling = true;
                    self.last_fall_time = Instant::now();
                    self.add_audio_event(AudioEvent::SoftDrop);
                }
            } else if can_fall_vertically {
                // Fallback: The diagonal is blocked, but we can fall straight down.
                // This prevents the card from getting stuck in mid-air.
                if let Some(card_mut) = self.current_card.as_mut() {
                    card_mut.target.x = current_pos.x; // Halt horizontal movement.
                    card_mut.target.y = next_y;
                    card_mut.is_falling = true;
                    self.last_fall_time = Instant::now();
                    self.add_audio_event(AudioEvent::SoftDrop);
                }
            } else {
                // Blocked below, even vertically. The card has landed.
                self.place_current_card();
            }
        } else {
            // No card to process, which can happen between placement and spawning.
        }
    }

    pub fn hard_drop(&mut self) {
        if let Some(mut current_card) = self.current_card.take() {
            // Calculate the final landing position by finding the lowest empty cell
            // Must check both board occupancy AND hard-dropping cards targeting the same position
            let mut final_y = current_card.position.y;
            let card_x = current_card.position.x;
            
            for test_y in (current_card.position.y + 1)..self.board.height {
                // Check if the board cell is empty
                let board_empty = self.board.is_cell_empty(card_x, test_y);
                
                // Check if any hard-dropping card is already targeting this position
                let no_hard_drop_conflict = !self.hard_dropping_cards.iter().any(|card| {
                    card.position.x == card_x && card.target.y == test_y
                });
                
                if board_empty && no_hard_drop_conflict {
                    final_y = test_y;
                } else {
                    // Hit an occupied cell or conflicting hard-drop target, stop here
                    break;
                }
            }

            // Only proceed if the card can actually fall
            if final_y > current_card.position.y {
                // Store the X position where the player was positioning this card
                // This ensures the next card spawns at the player's current position
                self.last_dropped_x = Some(current_card.position.x);
                
                // Set up the card for fast animated falling
                current_card.target.y = final_y;
                current_card.is_falling = true;
                current_card.is_hard_dropping = true;

                // Move the current card to the hard_dropping_cards list
                self.hard_dropping_cards.push(current_card);

                // Add audio event for hard drop
                self.add_audio_event(AudioEvent::HardDrop);

                // Immediately spawn a new card so the player can continue playing
                self.spawn_new_card();
            } else {
                // Card can't fall, place it immediately
                self.current_card = Some(current_card);
                self.place_current_card();
            }
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

            // Add audio event for dropping card
            self.add_audio_event(AudioEvent::DropCard);

            // Immediately process combinations after a card is placed.
            self.process_combinations();
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
        self.add_audio_event(AudioEvent::ReturnToGame);
    }

    pub fn transition_to_playing(&mut self) {
        self.state = Box::new(Playing);
        self.add_audio_event(AudioEvent::ResumeGame);
    }

    pub fn transition_to_paused(&mut self) {
        self.state = Box::new(Paused);
        self.add_audio_event(AudioEvent::PauseGame);
    }

    pub fn transition_to_game_over(&mut self) {
        self.state = Box::new(GameOver);
        self.add_audio_event(AudioEvent::GameOver);
    }

    pub fn transition_to_quit_confirm(&mut self) {
        self.state = Box::new(QuitConfirm);
        self.add_audio_event(AudioEvent::OpenQuitConfirmation);
    }

    // Audio event management
    pub fn add_audio_event(&mut self, event: AudioEvent) {
        self.pending_audio_events.push(event);
    }

    pub fn take_pending_audio_events(&mut self) -> Vec<AudioEvent> {
        std::mem::take(&mut self.pending_audio_events)
    }
}
