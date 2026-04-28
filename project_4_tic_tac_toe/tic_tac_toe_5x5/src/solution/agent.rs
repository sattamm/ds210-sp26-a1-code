use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // Count how many legal moves are left
        let moves_left = board.moves().len();

        // Choose how deep to search based on how full the board is
        let max_depth = if moves_left <= 6 {
            moves_left
        } else if moves_left <= 12 {
            5
        } else {
            4
        };

        // Start the recursive minmax search
        SolutionAgent::minmax(board, player, 0, max_depth, i32::MIN, i32::MAX)
    }
}

impl SolutionAgent {
    fn minmax(
        board: &mut Board,
        player: Player,
        depth: usize,
        max_depth: usize,
        mut lower_bound: i32,
        mut upper_bound: i32,
    ) -> (i32, usize, usize) {
        // If the game is finished, return a very high or low score
        if board.game_over() {
            let s = board.score();
            if s > 0 {
                // X wins, and winning sooner is slightly better
                return (10000 - depth as i32, 0, 0);
            } else if s < 0 {
                // O wins, and losing later is slightly better for X
                return (-10000 + depth as i32, 0, 0);
            } else {
                // Draw
                return (0, 0, 0);
            }
        }

        // If we reached the search limit, estimate board strength
        if depth == max_depth {
            return (SolutionAgent::heuristic(board), 0, 0);
        }

        // Get all possible legal moves
        let mut available_moves = board.moves();

        // Try moves near the center first
        let center = 2_i32;
        available_moves.sort_by_key(|pos| {
            let row_distance = (pos.0 as i32 - center).abs();
            let col_distance = (pos.1 as i32 - center).abs();
            row_distance + col_distance
        });

        // Start by assuming the first move is best
        let mut best_move = available_moves[0];

        match player {
            Player::X => {
                // X wants the highest score
                let mut best_score = i32::MIN;

                for moves in available_moves {
                    // Copy the board so this move does not affect other branches
                    let mut new_board = board.clone();
                    new_board.apply_move(moves, player);

                    // Recursively evaluate O's response
                    let (score, _, _) = SolutionAgent::minmax(
                        &mut new_board,
                        Player::O,
                        depth + 1,
                        max_depth,
                        lower_bound,
                        upper_bound,
                    );

                    // Keep track of the best score and move for X
                    if score > best_score {
                        best_score = score;
                        best_move = moves;
                    }

                    // Update the lower bound
                    lower_bound = lower_bound.max(best_score);

                    // Stop searching this branch if it cannot improve
                    if upper_bound <= lower_bound {
                        break;
                    }
                }

                // Return best score and best move for X
                (best_score, best_move.0, best_move.1)
            }

            Player::O => {
                // O wants the lowest score
                let mut best_score = i32::MAX;

                for moves in available_moves {
                    // Copy the board so this move does not affect other branches
                    let mut new_board = board.clone();
                    new_board.apply_move(moves, player);

                    // Recursively evaluate X's response
                    let (score, _, _) = SolutionAgent::minmax(
                        &mut new_board,
                        Player::X,
                        depth + 1,
                        max_depth,
                        lower_bound,
                        upper_bound,
                    );

                    // Keep track of the best score and move for O
                    if score < best_score {
                        best_score = score;
                        best_move = moves;
                    }

                    // Update the upper bound
                    upper_bound = upper_bound.min(best_score);

                    // Stop searching this branch if it cannot improve
                    if upper_bound <= lower_bound {
                        break;
                    }
                }

                // Return best score and best move for O
                (best_score, best_move.0, best_move.1)
            }
        }
    }

    fn heuristic(board: &Board) -> i32 {
        // Get the board's current score
        let current = board.score();

        // Get all legal next moves
        let available_moves = board.moves();

        // Start with the current board score
        let mut value = current * 10;

        for moves in available_moves {
            // See what happens if X plays this move
            let mut x_board = board.clone();
            x_board.apply_move(moves, Player::X);
            value += x_board.score();

            // See what happens if O plays this move
            let mut o_board = board.clone();
            o_board.apply_move(moves, Player::O);
            value -= o_board.score();
        }

        // Return the estimated value of the board
        value
    }
}