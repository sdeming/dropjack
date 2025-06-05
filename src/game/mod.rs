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

pub struct GameBuilder {
    board_width: i32,
    board_height: i32,
    cell_size: i32,
    difficulty: Difficulty,
    fall_speed: Duration,
    speed_increase_interval: Duration,
    database_path: Option<Box<Path>>,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            board_width: 10,
            board_height: 15,
            cell_size: 48,
            difficulty: Difficulty::Easy,
            fall_speed: Duration::from_millis(1000),
            speed_increase_interval: Duration::from_secs(30),
            database_path: None,
        }
    }

    // These builder methods provide valuable configuration options
    #[allow(dead_code)]
    pub fn board_size(mut self, width: i32, height: i32) -> Self {
        self.board_width = width;
        self.board_height = height;
        self
    }

    #[allow(dead_code)]
    pub fn cell_size(mut self, size: i32) -> Self {
        self.cell_size = size;
        self
    }

    #[allow(dead_code)]
    pub fn difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    #[allow(dead_code)]
    pub fn fall_speed(mut self, speed: Duration) -> Self {
        self.fall_speed = speed;
        self
    }

    #[allow(dead_code)]
    pub fn speed_increase_interval(mut self, interval: Duration) -> Self {
        self.speed_increase_interval = interval;
        self
    }

    pub fn database_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.database_path = Some(path.as_ref().into());
        self
    }

    pub fn build(self) -> Result<Game, Box<dyn std::error::Error>> {
        let mut deck = Deck::new();
        deck.shuffle();

        let board = Board::new(self.board_width, self.board_height, self.cell_size);

        let database = Database::new(
            self.database_path
                .as_ref()
                .ok_or("Database path must be provided")?
        )?;
        let high_scores = database.get_high_scores(10).unwrap_or_default();

        let next_card = deck.draw();
        let now = Instant::now();

        Ok(Game {
            state: Box::new(StartScreen),
            board,
            deck,
            current_card: None,
            next_card,
            score: 0,
            difficulty: self.difficulty,
            fall_speed: self.fall_speed,
            last_fall_time: now,
            speed_increase_interval: self.speed_increase_interval,
            last_speed_increase: now,
            database,
            high_scores,
            player_initials: String::new(),
            pending_explosions: Vec::new(),
            delayed_destructions: Vec::new(),
            last_dropped_x: None,
            pending_audio_events: Vec::new(),
            hard_dropping_cards: Vec::new(),
        })
    }
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
    pub fn builder() -> GameBuilder {
        GameBuilder::new()
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
            
            self.current_card = Some(
                PlayingCard::builder(card, position)
                    .cell_size(self.board.cell_size)
                    .visual_position(VisualPosition {
                        x: (x * self.board.cell_size) as f32,
                        y: 0f32,
                    })
                    .target(Position { x, y: 0 })
                    .falling(false)
                    .hard_dropping(false)
                    .build()
            );
            
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // Test fixtures for game testing
    mod test_fixtures {
        use super::*;
        use tempfile;

        pub fn create_temp_database() -> (Database, TempDir) {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
            let db_path = temp_dir.path().join("test_game.db");
            let db = Database::new(&db_path).expect("Failed to create test database");
            (db, temp_dir)
        }

        pub fn create_test_game() -> (Game, TempDir) {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
            let db_path = temp_dir.path().join("test_game.db");
            
            let game = Game::builder()
                .database_path(&db_path)
                .build()
                .expect("Failed to create test game");
                
            (game, temp_dir)
        }

        pub fn create_test_game_with_config(
            width: i32,
            height: i32,
            difficulty: Difficulty,
        ) -> (Game, TempDir) {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
            let db_path = temp_dir.path().join("test_game.db");
            
            let game = Game::builder()
                .board_size(width, height)
                .difficulty(difficulty)
                .database_path(&db_path)
                .build()
                .expect("Failed to create test game");
                
            (game, temp_dir)
        }

        pub fn create_test_playing_card() -> PlayingCard {
            PlayingCard::builder(
                Card::new(crate::models::Suit::Hearts, crate::models::Value::Ace),
                Position { x: 2, y: 1 }
            ).build()
        }
    }

    #[test]
    fn test_game_builder_basic() {
        let (game, _temp_dir) = test_fixtures::create_test_game();
        
        assert_eq!(game.board.width, 10); // Default width
        assert_eq!(game.board.height, 15); // Default height
        assert_eq!(game.difficulty, Difficulty::Easy); // Default difficulty
        assert_eq!(game.score, 0);
        assert!(game.current_card.is_none());
        assert!(game.next_card.is_some());
        assert!(game.is_start_screen());
    }

    #[test]
    fn test_game_builder_with_custom_config() {
        let (game, _temp_dir) = test_fixtures::create_test_game_with_config(8, 12, Difficulty::Hard);
        
        assert_eq!(game.board.width, 8);
        assert_eq!(game.board.height, 12);
        assert_eq!(game.difficulty, Difficulty::Hard);
    }

    #[test]
    fn test_game_builder_all_options() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test_game.db");
        
        let game = Game::builder()
            .board_size(6, 10)
            .cell_size(32)
            .difficulty(Difficulty::Hard)
            .fall_speed(Duration::from_millis(500))
            .speed_increase_interval(Duration::from_secs(45))
            .database_path(&db_path)
            .build()
            .expect("Failed to create game");
        
        assert_eq!(game.board.width, 6);
        assert_eq!(game.board.height, 10);
        assert_eq!(game.board.cell_size, 32);
        assert_eq!(game.difficulty, Difficulty::Hard);
        assert_eq!(game.fall_speed, Duration::from_millis(500));
        assert_eq!(game.speed_increase_interval, Duration::from_secs(45));
    }

    #[test]
    fn test_game_builder_missing_database_path() {
        let result = Game::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_game_state_transitions() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        // Should start in StartScreen state
        assert!(game.is_start_screen());
        assert!(!game.is_playing());
        assert!(!game.is_paused());
        assert!(!game.is_game_over());
        assert!(!game.is_quit_confirm());

        // Transition to playing
        game.transition_to_playing();
        assert!(game.is_playing());
        assert!(!game.is_start_screen());

        // Transition to paused
        game.transition_to_paused();
        assert!(game.is_paused());
        assert!(!game.is_playing());

        // Transition to game over
        game.transition_to_game_over();
        assert!(game.is_game_over());
        assert!(!game.is_paused());

        // Transition to quit confirm
        game.transition_to_quit_confirm();
        assert!(game.is_quit_confirm());
        assert!(!game.is_game_over());

        // Transition back to start screen
        game.transition_to_start_screen();
        assert!(game.is_start_screen());
        assert!(!game.is_quit_confirm());
    }

    #[test]
    fn test_start_game() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        game.start_game(Difficulty::Hard);
        
        assert!(game.is_playing());
        assert_eq!(game.difficulty, Difficulty::Hard);
        assert_eq!(game.score, 0);
        assert_eq!(game.fall_speed, Duration::from_millis(1000));
        assert!(game.current_card.is_some());
        assert!(!game.pending_audio_events.is_empty());
        
        // Should have StartGame audio event
        let audio_events = game.take_pending_audio_events();
        assert!(audio_events.contains(&AudioEvent::StartGame));
    }

    #[test]
    fn test_spawn_new_card() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        // Ensure we have a next card
        assert!(game.next_card.is_some());
        
        game.spawn_new_card();
        
        assert!(game.current_card.is_some());
        
        let current_card = game.current_card.as_ref().unwrap();
        assert_eq!(current_card.position.y, 0); // Should spawn at top
        assert!(current_card.position.x >= 0 && current_card.position.x < game.board.width);
    }

    #[test]
    fn test_move_current_card_left() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        game.current_card = Some(test_fixtures::create_test_playing_card());
        
        let initial_x = game.current_card.as_ref().unwrap().position.x;
        game.move_current_card_left();
        
        if initial_x > 0 {
            // Movement updates target, not position
            assert_eq!(game.current_card.as_ref().unwrap().target.x, initial_x - 1);
            
            // Should have MoveLeft audio event
            let audio_events = game.take_pending_audio_events();
            assert!(audio_events.contains(&AudioEvent::MoveLeft));
        }
    }

    #[test]
    fn test_move_current_card_right() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        game.current_card = Some(test_fixtures::create_test_playing_card());
        
        let initial_x = game.current_card.as_ref().unwrap().position.x;
        game.move_current_card_right();
        
        if initial_x < game.board.width - 1 {
            // Movement updates target, not position
            assert_eq!(game.current_card.as_ref().unwrap().target.x, initial_x + 1);
            
            // Should have MoveRight audio event
            let audio_events = game.take_pending_audio_events();
            assert!(audio_events.contains(&AudioEvent::MoveRight));
        }
    }

    #[test]
    fn test_move_current_card_down() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        let mut card = test_fixtures::create_test_playing_card();
        card.position.y = 5; // Not at bottom
        game.current_card = Some(card);
        
        let initial_y = game.current_card.as_ref().unwrap().position.y;
        game.move_current_card_down();
        
        if initial_y < game.board.height - 1 {
            // Should have moved down
            if game.current_card.is_some() {
                // Still have card (it moved but didn't land)
                let audio_events = game.take_pending_audio_events();
                assert!(audio_events.contains(&AudioEvent::SoftDrop));
            } else {
                // Card was placed (reached bottom or landed on something)
                let audio_events = game.take_pending_audio_events();
                assert!(audio_events.contains(&AudioEvent::DropCard));
            }
        }
    }

    #[test]
    fn test_is_move_valid() {
        let (game, _temp_dir) = test_fixtures::create_test_game();
        
        // Valid moves within bounds
        assert!(game.is_move_valid(2, 2, 3, 2)); // Right
        assert!(game.is_move_valid(2, 2, 1, 2)); // Left
        assert!(game.is_move_valid(2, 2, 2, 3)); // Down
        
        // Invalid moves out of bounds
        assert!(!game.is_move_valid(0, 0, -1, 0)); // Left out of bounds
        assert!(!game.is_move_valid(9, 0, 10, 0)); // Right out of bounds (width=10)
        assert!(!game.is_move_valid(0, 14, 0, 15)); // Down out of bounds (height=15)
        
        // Note: is_move_valid only checks bounds and emptiness, not game rules like "no up movement"
        // Game rules are enforced by the higher-level movement functions
        assert!(game.is_move_valid(2, 2, 2, 1)); // Up is valid in terms of board bounds
    }

    #[test]
    fn test_hard_drop() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        let mut card = test_fixtures::create_test_playing_card();
        card.position.y = 1; // Near top
        game.current_card = Some(card);
        
        game.hard_drop();
        
        // Card should be placed at bottom and new card spawned
        assert!(game.current_card.is_some()); // New card spawned
        
        // Should have HardDrop audio event
        let audio_events = game.take_pending_audio_events();
        assert!(audio_events.contains(&AudioEvent::HardDrop));
    }

    #[test]
    fn test_add_initial() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        game.add_initial('A');
        game.add_initial('B');
        game.add_initial('C');
        
        assert_eq!(game.player_initials, "ABC");
        
        // Should limit to 3 characters
        game.add_initial('D');
        assert_eq!(game.player_initials, "ABC");
    }

    #[test]
    fn test_remove_initial() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        game.player_initials = "ABC".to_string();
        
        game.remove_initial();
        assert_eq!(game.player_initials, "AB");
        
        game.remove_initial();
        assert_eq!(game.player_initials, "A");
        
        game.remove_initial();
        assert_eq!(game.player_initials, "");
        
        // Should not crash when empty
        game.remove_initial();
        assert_eq!(game.player_initials, "");
    }

    #[test]
    fn test_audio_events() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        game.add_audio_event(AudioEvent::DropCard);
        game.add_audio_event(AudioEvent::MakeMatch);
        
        let events = game.take_pending_audio_events();
        assert_eq!(events.len(), 2);
        assert!(events.contains(&AudioEvent::DropCard));
        assert!(events.contains(&AudioEvent::MakeMatch));
        
        // Should be empty after taking
        let events2 = game.take_pending_audio_events();
        assert!(events2.is_empty());
    }

    #[test]
    fn test_take_pending_explosions() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        let card = Card::new(crate::models::Suit::Hearts, crate::models::Value::King);
        game.pending_explosions.push((1, 2, card));
        game.pending_explosions.push((3, 4, card));
        
        let explosions = game.take_pending_explosions();
        assert_eq!(explosions.len(), 2);
        assert_eq!(explosions[0], (1, 2, card));
        assert_eq!(explosions[1], (3, 4, card));
        
        // Should be empty after taking
        let explosions2 = game.take_pending_explosions();
        assert!(explosions2.is_empty());
    }

    #[test]
    fn test_increase_speed() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        let initial_speed = game.fall_speed;
        
        game.increase_speed();
        
        assert!(game.fall_speed < initial_speed);
        assert!(game.fall_speed >= Duration::from_millis(100)); // Should not go below minimum
    }

    #[test]
    fn test_save_high_score() {
        let (mut game, _temp_dir) = test_fixtures::create_test_game();
        
        game.player_initials = "TST".to_string();
        game.score = 1500;
        game.difficulty = Difficulty::Hard;
        
        game.save_high_score();
        
        // Should reload high scores
        assert!(!game.high_scores.is_empty());
        
        // Find our score
        let our_score = game.high_scores.iter()
            .find(|hs| hs.player_initials == "TST" && hs.score == 1500)
            .expect("Should find our high score");
        
        assert_eq!(our_score.difficulty, "Hard");
    }

    #[test]
    fn test_audio_event_enum_properties() {
        // Test that AudioEvent implements required traits
        let event1 = AudioEvent::StartGame;
        let event2 = AudioEvent::StartGame;
        let event3 = AudioEvent::DropCard;
        
        // Test Debug
        assert!(!format!("{:?}", event1).is_empty());
        
        // Test Clone
        let cloned = event1.clone();
        assert_eq!(event1, cloned);
        
        // Test Copy
        let copied = event1;
        assert_eq!(event1, copied);
        
        // Test Hash and Eq
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(event1, "test");
        assert!(map.contains_key(&event2));
        assert!(!map.contains_key(&event3));
        
        // Test PartialEq
        assert_eq!(event1, event2);
        assert_ne!(event1, event3);
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_full_game_flow() {
            let (mut game, _temp_dir) = test_fixtures::create_test_game();
            
            // Start game
            game.start_game(Difficulty::Easy);
            assert!(game.is_playing());
            
            // Should have a current card
            assert!(game.current_card.is_some());
            
            // Move card around
            game.move_current_card_left();
            game.move_current_card_right();
            game.move_current_card_down();
            
            // Hard drop
            game.hard_drop();
            
            // Should still have a card (new one spawned)
            assert!(game.current_card.is_some());
            
            // Transition states
            game.transition_to_paused();
            assert!(game.is_paused());
            
            game.transition_to_playing();
            assert!(game.is_playing());
            
            // End game
            game.transition_to_game_over();
            assert!(game.is_game_over());
            
            // Add initials and save score
            game.add_initial('T');
            game.add_initial('S');
            game.add_initial('T');
            game.save_high_score();
            
            // Return to start
            game.transition_to_start_screen();
            assert!(game.is_start_screen());
        }

        #[test]
        fn test_game_persistence() {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
            let db_path = temp_dir.path().join("persistence_test.db");
            
            // Create game and save a score
            {
                let mut game = Game::builder()
                    .database_path(&db_path)
                    .build()
                    .expect("Failed to create game");
                
                game.player_initials = "PER".to_string();
                game.score = 999;
                game.save_high_score();
            }
            
            // Create new game instance and verify score persists
            {
                let game = Game::builder()
                    .database_path(&db_path)
                    .build()
                    .expect("Failed to create game");
                
                assert!(!game.high_scores.is_empty());
                let persistent_score = game.high_scores.iter()
                    .find(|hs| hs.player_initials == "PER" && hs.score == 999);
                assert!(persistent_score.is_some());
            }
        }

        #[test]
        fn test_difficulty_variations() {
            for difficulty in [Difficulty::Easy, Difficulty::Hard] {
                let (game, _temp_dir) = test_fixtures::create_test_game_with_config(8, 12, difficulty);
                assert_eq!(game.difficulty, difficulty);
                assert_eq!(game.board.width, 8);
                assert_eq!(game.board.height, 12);
            }
        }
    }
}
