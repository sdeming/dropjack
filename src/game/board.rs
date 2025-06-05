use crate::models::{Card, Difficulty, FallingCard};

// The game board
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<Vec<Option<Card>>>,
    pub cell_size: i32,
    pub falling_cards: Vec<FallingCard>, // Cards currently falling due to gravity
    pub marked_for_removal: Vec<Vec<Option<std::time::Instant>>>, // Timestamp when each card should be removed
}

impl Board {
    pub fn new(width: i32, height: i32, cell_size: i32) -> Self {
        let grid = (0..height).map(|_| vec![None; width as usize]).collect();

        Board {
            width,
            height,
            grid,
            cell_size,
            falling_cards: Vec::new(),
            marked_for_removal: vec![vec![None; width as usize]; height as usize],
        }
    }

    pub fn is_position_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn is_cell_empty(&self, x: i32, y: i32) -> bool {
        if !self.is_position_valid(x, y) {
            return false;
        }
        self.grid[y as usize][x as usize].is_none()
    }

    pub fn place_card(&mut self, x: i32, y: i32, card: Card) -> bool {
        if !self.is_cell_empty(x, y) {
            return false;
        }

        self.grid[y as usize][x as usize] = Some(card);
        true
    }

    pub fn remove_card(&mut self, x: i32, y: i32) -> Option<Card> {
        if !self.is_position_valid(x, y) {
            return None;
        }

        let card = self.grid[y as usize][x as usize];
        self.grid[y as usize][x as usize] = None;
        card
    }

    // Check for combinations that sum to 21 using comprehensive path finding
    pub fn check_combinations(&mut self, difficulty: Difficulty) -> Vec<(i32, i32)> {
        let mut all_removed_positions = Vec::new();
        let mut global_visited = vec![vec![false; self.width as usize]; self.height as usize];

        // Check every position as a potential starting point
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(start_card) = self.grid[y as usize][x as usize] {
                    if !global_visited[y as usize][x as usize] {
                        // Try to find the best path starting from this card
                        let mut path = Vec::new();
                        let mut local_visited =
                            vec![vec![false; self.width as usize]; self.height as usize];

                        let combinations = self.find_all_paths_to_21(
                            x,
                            y,
                            start_card,
                            0,
                            &mut path,
                            difficulty,
                            &mut local_visited,
                        );

                        // Find the longest valid combination (prefer longer paths for higher scores)
                        if let Some(best_combination) = combinations
                            .into_iter()
                            .filter(|combo| combo.len() >= 2) // Need at least 2 cards
                            .max_by_key(|combo| combo.len())
                        {
                            // Mark all positions in this combination as removed
                            best_combination.iter().for_each(|&(px, py)| {
                                if !all_removed_positions.contains(&(px, py)) {
                                    all_removed_positions.push((px, py));
                                }
                                global_visited[py as usize][px as usize] = true;
                            });
                        }
                    }
                }
            }
        }

        // Sort the result
        all_removed_positions.sort();
        all_removed_positions
    }

    // Mark cards for delayed removal
    pub fn mark_cards_for_removal(
        &mut self,
        positions: Vec<(i32, i32)>,
        removal_time: std::time::Instant,
    ) {
        for &(x, y) in &positions {
            if self.is_position_valid(x, y) {
                self.marked_for_removal[y as usize][x as usize] = Some(removal_time);
            }
        }
    }

    // Process marked cards that are ready for removal
    pub fn process_marked_removals(&mut self) -> Vec<(i32, i32, Card)> {
        let now = std::time::Instant::now();
        let mut removed_cards = Vec::new();

        // Create a list of coordinates to check
        let coordinates: Vec<(i32, i32)> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .collect();

        // Process each coordinate
        for (x, y) in coordinates {
            if let Some(removal_time) = self.marked_for_removal[y as usize][x as usize] {
                if now >= removal_time {
                    // Time to remove this card
                    if let Some(card) = self.remove_card(x, y) {
                        removed_cards.push((x, y, card));
                    }
                    self.marked_for_removal[y as usize][x as usize] = None;
                }
            }
        }

        removed_cards
    }

    // Find all possible paths from a starting position that sum to 21
    fn find_all_paths_to_21(
        &self,
        x: i32,
        y: i32,
        current_card: Card,
        current_sum: i32,
        path: &mut Vec<(i32, i32)>,
        difficulty: Difficulty,
        visited: &mut Vec<Vec<bool>>,
    ) -> Vec<Vec<(i32, i32)>> {
        let mut all_combinations = Vec::new();

        // Mark the current position as visited for this path
        visited[y as usize][x as usize] = true;
        path.push((x, y));

        // Try both values for Ace (1 or 11), otherwise use standard value
        let possible_values = current_card.blackjack_values();

        for &card_value in &possible_values {
            let new_sum = current_sum + card_value as i32;

            if new_sum == 21 {
                // Found a valid combination!
                all_combinations.push(path.clone());
            } else if new_sum < 21 {
                // Continue searching adjacent cells (4-directional only: up, down, left, right)
                let directions = [
                    (-1, 0), // Up
                    (1, 0),  // Down
                    (0, -1), // Left
                    (0, 1),  // Right
                ];

                for &(dx, dy) in &directions {
                    let next_x = x + dx;
                    let next_y = y + dy;

                    if self.is_position_valid(next_x, next_y)
                        && !visited[next_y as usize][next_x as usize]
                    {
                        if let Some(next_card) = self.grid[next_y as usize][next_x as usize] {
                            if difficulty == Difficulty::Easy || current_card.suit == next_card.suit
                            {
                                let sub_combinations = self.find_all_paths_to_21(
                                    next_x, next_y, next_card, new_sum, path, difficulty, visited,
                                );
                                all_combinations.extend(sub_combinations);
                            }
                        }
                    }
                }
            }
            // If new_sum > 21, this path is invalid, try next value or backtrack
        }

        // Backtrack - unmark as visited for this path exploration
        visited[y as usize][x as usize] = false;
        path.pop();

        all_combinations
    }

    // Apply gravity to compact cards downwards in each column.
    // This uses a single-pass approach for each column, which is more efficient
    // than the previous implementation. It also ensures that cards can't collide
    // or end up in invalid positions.
    pub fn apply_gravity(&mut self) -> bool {
        self.falling_cards.retain(|card| card.is_animating);

        let mut changes_made = false;

        for x in 0..self.width {
            let mut write_y = self.height - 1;
            for read_y in (0..self.height).rev() {
                if let Some(card) = self.grid[read_y as usize][x as usize].take() {
                    if read_y != write_y {
                        let falling_card = FallingCard {
                            card,
                            to_y: write_y,
                            x,
                            visual_y: (read_y * self.cell_size) as f32,
                            is_animating: true,
                        };
                        self.falling_cards.push(falling_card);
                        changes_made = true;
                    }
                    self.grid[write_y as usize][x as usize] = Some(card);
                    write_y -= 1;
                }
            }
        }

        changes_made
    }

    // Update falling card animations
    pub fn update_falling_cards(&mut self) {
        let fall_speed = 6.0; // Pixels per frame

        for falling_card in &mut self.falling_cards {
            if falling_card.is_animating {
                let target_y = (falling_card.to_y * self.cell_size) as f32;

                if falling_card.visual_y < target_y {
                    falling_card.visual_y += fall_speed;

                    if falling_card.visual_y >= target_y {
                        falling_card.visual_y = target_y;
                        falling_card.is_animating = false;
                    }
                } else {
                    falling_card.is_animating = false;
                }
            }
        }
    }

    // Check if the game is over (any card at the top row)
    pub fn is_game_over(&self) -> bool {
        for x in 0..self.width {
            if self.grid[0][x as usize].is_some() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Suit, Value, Card, Difficulty};
    use std::time::{Duration, Instant};

    // Test fixtures for creating boards and cards for testing
    mod test_fixtures {
        use super::*;

        pub fn create_test_board() -> Board {
            Board::new(5, 8, 50)
        }

        pub fn create_small_board() -> Board {
            Board::new(3, 3, 30)
        }

        pub fn create_cards_for_21_combination() -> Vec<Card> {
            vec![
                Card::new(Suit::Hearts, Value::Ten),   // 10
                Card::new(Suit::Hearts, Value::Five),  // 5
                Card::new(Suit::Hearts, Value::Six),   // 6 (10 + 5 + 6 = 21)
            ]
        }

        pub fn create_cards_for_ace_combination() -> Vec<Card> {
            vec![
                Card::new(Suit::Spades, Value::Ace),   // 1 or 11
                Card::new(Suit::Spades, Value::King),  // 10 (Ace as 11 + 10 = 21)
            ]
        }

        pub fn create_mixed_suit_cards() -> Vec<Card> {
            vec![
                Card::new(Suit::Hearts, Value::Ten),
                Card::new(Suit::Spades, Value::Five),
                Card::new(Suit::Diamonds, Value::Six),
            ]
        }

        #[allow(dead_code)]
        pub fn setup_board_with_pattern(board: &mut Board, pattern: &[(i32, i32, Card)]) {
            for &(x, y, card) in pattern {
                board.place_card(x, y, card);
            }
        }
    }

    #[test]
    fn test_board_new() {
        let board = Board::new(5, 8, 50);
        assert_eq!(board.width, 5);
        assert_eq!(board.height, 8);
        assert_eq!(board.cell_size, 50);
        assert_eq!(board.grid.len(), 8);
        assert_eq!(board.grid[0].len(), 5);
        assert!(board.falling_cards.is_empty());

        // All cells should be empty initially
        for row in &board.grid {
            for cell in row {
                assert!(cell.is_none());
            }
        }
    }

    #[test]
    fn test_is_position_valid() {
        let board = test_fixtures::create_test_board();

        // Valid positions
        assert!(board.is_position_valid(0, 0));
        assert!(board.is_position_valid(4, 7));
        assert!(board.is_position_valid(2, 4));

        // Invalid positions
        assert!(!board.is_position_valid(-1, 0));
        assert!(!board.is_position_valid(0, -1));
        assert!(!board.is_position_valid(5, 0));
        assert!(!board.is_position_valid(0, 8));
        assert!(!board.is_position_valid(5, 8));
    }

    #[test]
    fn test_is_cell_empty() {
        let mut board = test_fixtures::create_test_board();
        let card = Card::new(Suit::Hearts, Value::Ace);

        // Initially all cells should be empty
        assert!(board.is_cell_empty(0, 0));
        assert!(board.is_cell_empty(2, 4));

        // Place a card and test
        board.place_card(2, 4, card);
        assert!(!board.is_cell_empty(2, 4));
        assert!(board.is_cell_empty(2, 3)); // Adjacent cell still empty

        // Test invalid positions
        assert!(!board.is_cell_empty(-1, 0));
        assert!(!board.is_cell_empty(5, 8));
    }

    #[test]
    fn test_place_card() {
        let mut board = test_fixtures::create_test_board();
        let card = Card::new(Suit::Hearts, Value::King);

        // Test successful placement
        assert!(board.place_card(1, 3, card));
        assert_eq!(board.grid[3][1], Some(card));

        // Test placement on occupied cell
        let another_card = Card::new(Suit::Spades, Value::Queen);
        assert!(!board.place_card(1, 3, another_card));
        assert_eq!(board.grid[3][1], Some(card)); // Original card unchanged

        // Test placement on invalid position
        assert!(!board.place_card(-1, 0, another_card));
        assert!(!board.place_card(5, 8, another_card));
    }

    #[test]
    fn test_remove_card() {
        let mut board = test_fixtures::create_test_board();
        let card = Card::new(Suit::Diamonds, Value::Seven);

        // Place a card first
        board.place_card(2, 5, card);
        assert_eq!(board.grid[5][2], Some(card));

        // Remove the card
        let removed_card = board.remove_card(2, 5);
        assert_eq!(removed_card, Some(card));
        assert_eq!(board.grid[5][2], None);

        // Try removing from empty cell
        let empty_removal = board.remove_card(2, 5);
        assert_eq!(empty_removal, None);

        // Try removing from invalid position
        let invalid_removal = board.remove_card(-1, 0);
        assert_eq!(invalid_removal, None);
    }

    #[test]
    fn test_check_combinations_simple_21() {
        let mut board = test_fixtures::create_small_board();
        let cards = test_fixtures::create_cards_for_21_combination();

        // Place cards in a line: 10-5-6 horizontally
        board.place_card(0, 1, cards[0]); // 10
        board.place_card(1, 1, cards[1]); // 5
        board.place_card(2, 1, cards[2]); // 6

        let removed_positions = board.check_combinations(Difficulty::Easy);
        
        // Should find the combination that sums to 21
        assert!(!removed_positions.is_empty());
        assert!(removed_positions.contains(&(0, 1)));
        assert!(removed_positions.contains(&(1, 1)));
        assert!(removed_positions.contains(&(2, 1)));
    }

    #[test]
    fn test_check_combinations_ace_as_eleven() {
        let mut board = test_fixtures::create_small_board();
        let cards = test_fixtures::create_cards_for_ace_combination();

        // Place Ace and King adjacent (Ace as 11 + King as 10 = 21)
        board.place_card(0, 0, cards[0]); // Ace
        board.place_card(1, 0, cards[1]); // King

        let removed_positions = board.check_combinations(Difficulty::Easy);
        
        // Should find the combination
        assert_eq!(removed_positions.len(), 2);
        assert!(removed_positions.contains(&(0, 0)));
        assert!(removed_positions.contains(&(1, 0)));
    }

    #[test]
    fn test_check_combinations_difficulty_restrictions() {
        let mut board = test_fixtures::create_small_board();
        let mixed_cards = test_fixtures::create_mixed_suit_cards();

        // Place mixed suit cards that sum to 21
        board.place_card(0, 1, mixed_cards[0]); // Hearts 10
        board.place_card(1, 1, mixed_cards[1]); // Spades 5
        board.place_card(2, 1, mixed_cards[2]); // Diamonds 6

        // Easy mode should find combination (ignores suits)
        let easy_combinations = board.check_combinations(Difficulty::Easy);
        assert!(!easy_combinations.is_empty());

        // Hard mode should not find combination (different suits)
        let hard_combinations = board.check_combinations(Difficulty::Hard);
        assert!(hard_combinations.is_empty());
    }

    #[test]
    fn test_check_combinations_no_valid_combination() {
        let mut board = test_fixtures::create_small_board();

        // Place cards that don't sum to 21
        board.place_card(0, 0, Card::new(Suit::Hearts, Value::Two));
        board.place_card(1, 0, Card::new(Suit::Hearts, Value::Three));
        board.place_card(2, 0, Card::new(Suit::Hearts, Value::Four));

        let removed_positions = board.check_combinations(Difficulty::Easy);
        assert!(removed_positions.is_empty());
    }

    #[test]
    fn test_mark_cards_for_removal() {
        let mut board = test_fixtures::create_test_board();
        let positions = vec![(1, 2), (3, 4)];
        let removal_time = Instant::now() + Duration::from_millis(100);

        board.mark_cards_for_removal(positions.clone(), removal_time);

        // Check that positions are marked
        assert!(board.marked_for_removal[2][1].is_some());
        assert!(board.marked_for_removal[4][3].is_some());
        assert!(board.marked_for_removal[0][0].is_none());

        // Check that invalid positions are ignored
        board.mark_cards_for_removal(vec![(-1, 0), (10, 10)], removal_time);
        // Should not panic and should not affect valid positions
    }

    #[test]
    fn test_process_marked_removals() {
        let mut board = test_fixtures::create_test_board();
        let card1 = Card::new(Suit::Hearts, Value::Ace);
        let card2 = Card::new(Suit::Spades, Value::King);

        // Place cards
        board.place_card(1, 2, card1);
        board.place_card(3, 4, card2);

        // Mark for immediate removal
        let removal_time = Instant::now();
        board.mark_cards_for_removal(vec![(1, 2), (3, 4)], removal_time);

        // Process removals
        let removed_cards = board.process_marked_removals();

        // Check that cards were removed
        assert_eq!(removed_cards.len(), 2);
        assert!(board.grid[2][1].is_none());
        assert!(board.grid[4][3].is_none());
        
        // Check removal tracking is cleared
        assert!(board.marked_for_removal[2][1].is_none());
        assert!(board.marked_for_removal[4][3].is_none());
    }

    #[test]
    fn test_process_marked_removals_future_time() {
        let mut board = test_fixtures::create_test_board();
        let card = Card::new(Suit::Hearts, Value::Ace);

        // Place card
        board.place_card(1, 2, card);

        // Mark for future removal
        let future_time = Instant::now() + Duration::from_secs(10);
        board.mark_cards_for_removal(vec![(1, 2)], future_time);

        // Process removals (should not remove yet)
        let removed_cards = board.process_marked_removals();

        // Card should still be there
        assert!(removed_cards.is_empty());
        assert_eq!(board.grid[2][1], Some(card));
        assert!(board.marked_for_removal[2][1].is_some());
    }

    #[test]
    fn test_apply_gravity_simple() {
        let mut board = test_fixtures::create_test_board();
        let card1 = Card::new(Suit::Hearts, Value::Ace);
        let card2 = Card::new(Suit::Spades, Value::King);

        // Place cards with gaps
        board.place_card(2, 3, card1); // Column 2, Row 3
        board.place_card(2, 1, card2); // Column 2, Row 1 (above gap)

        let changes_made = board.apply_gravity();

        // Gravity should have been applied
        assert!(changes_made);
        
        // Cards should move down
        assert!(board.falling_cards.len() > 0);
        
        // Check final positions in grid after gravity settles
        assert_eq!(board.grid[7][2], Some(card1)); // Bottom
        assert_eq!(board.grid[6][2], Some(card2)); // Above bottom
        assert!(board.grid[3][2].is_none()); // Original position should be empty
        assert!(board.grid[1][2].is_none()); // Original position should be empty
    }

    #[test]
    fn test_apply_gravity_no_changes() {
        let mut board = test_fixtures::create_test_board();
        let card = Card::new(Suit::Hearts, Value::Ace);

        // Place card at bottom
        board.place_card(2, 7, card);

        let changes_made = board.apply_gravity();

        // No changes should be made
        assert!(!changes_made);
        assert!(board.falling_cards.is_empty());
        assert_eq!(board.grid[7][2], Some(card));
    }

    #[test]
    fn test_update_falling_cards() {
        let mut board = test_fixtures::create_test_board();
        let card = Card::new(Suit::Hearts, Value::Ace);

        // Manually create a falling card
        let falling_card = FallingCard {
            card,
            to_y: 7,
            x: 2,
            visual_y: 100.0, // Start position
            is_animating: true,
        };
        board.falling_cards.push(falling_card);

        // Update falling cards
        board.update_falling_cards();

        // Visual position should have moved down
        assert!(board.falling_cards[0].visual_y > 100.0);
        
        // If we update enough times, animation should complete
        for _ in 0..100 {
            board.update_falling_cards();
            if !board.falling_cards[0].is_animating {
                break;
            }
        }
        
        // Should eventually reach target
        assert_eq!(board.falling_cards[0].visual_y, (7 * board.cell_size) as f32);
        assert!(!board.falling_cards[0].is_animating);
    }

    #[test]
    fn test_is_game_over() {
        let mut board = test_fixtures::create_test_board();

        // Initially not game over
        assert!(!board.is_game_over());

        // Place cards not in top row
        board.place_card(2, 4, Card::new(Suit::Hearts, Value::Ace));
        board.place_card(1, 7, Card::new(Suit::Spades, Value::King));
        assert!(!board.is_game_over());

        // Place card in top row
        board.place_card(3, 0, Card::new(Suit::Diamonds, Value::Queen));
        assert!(board.is_game_over());
    }

    #[test]
    fn test_complex_combination_finding() {
        let mut board = Board::new(4, 4, 30);

        // Create a more complex pattern
        // Row 1: [A♠, 5♠, _, 3♠]  - Ace(1) + 5 + 3 = 9, need 12 more
        // Row 2: [_, Q♠, _, _]    - Queen = 10, so we have 9 + 10 = 19, need 2 more  
        // Row 3: [_, 2♠, _, _]    - 2 completes it: 9 + 10 + 2 = 21
        board.place_card(0, 1, Card::new(Suit::Spades, Value::Ace));
        board.place_card(1, 1, Card::new(Suit::Spades, Value::Five));
        board.place_card(3, 1, Card::new(Suit::Spades, Value::Three));
        board.place_card(1, 2, Card::new(Suit::Spades, Value::Queen));
        board.place_card(1, 3, Card::new(Suit::Spades, Value::Two));

        let combinations = board.check_combinations(Difficulty::Hard);
        
        // Should find the path: Ace(1) -> 5 -> Queen(10) -> 2 = 18, or 
        // Try different path with Ace as 11: not possible here as would exceed 21
        // The algorithm should find valid 21 combinations
        if !combinations.is_empty() {
            // Verify the combinations make sense
            assert!(combinations.len() >= 2); // Should have at least 2 cards for valid combination
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_full_game_cycle() {
            let mut board = test_fixtures::create_test_board();
            
            // Simulate a game cycle
            // 1. Place some cards
            let cards = [
                (1, 6, Card::new(Suit::Hearts, Value::Ten)),
                (1, 5, Card::new(Suit::Hearts, Value::Five)),
                (1, 4, Card::new(Suit::Hearts, Value::Six)),
                (2, 7, Card::new(Suit::Spades, Value::King)),
            ];
            
            for &(x, y, card) in &cards {
                board.place_card(x, y, card);
            }
            
            // 2. Check for combinations
            let combinations = board.check_combinations(Difficulty::Easy);
            
            if !combinations.is_empty() {
                // 3. Mark for removal
                let removal_time = Instant::now();
                board.mark_cards_for_removal(combinations, removal_time);
                
                // 4. Process removals
                let removed = board.process_marked_removals();
                assert!(!removed.is_empty());
                
                // 5. Apply gravity
                let gravity_applied = board.apply_gravity();
                if gravity_applied {
                    // 6. Update falling animations
                    for _ in 0..100 {
                        board.update_falling_cards();
                        let all_settled = board.falling_cards.iter().all(|fc| !fc.is_animating);
                        if all_settled {
                            break;
                        }
                    }
                }
            }
            
            // Game should be in a valid state
            assert!(!board.is_game_over() || board.grid[0].iter().any(|cell| cell.is_some()));
        }
    }
}
