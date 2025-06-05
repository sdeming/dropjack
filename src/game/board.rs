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
