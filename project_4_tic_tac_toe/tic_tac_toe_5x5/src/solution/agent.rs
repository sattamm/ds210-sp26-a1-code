use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    fn heuristic(board: &Board) -> i32 {
        board.score()
    }

    fn minimax(board: &Board, player: Player, depth: usize, max_depth: usize) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0);
        }

        if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }

        let moves = board.moves();

        if moves.is_empty() {
            return (board.score(), 0, 0);
        }

        let mut best_score = if player == Player::X {
            i32::MIN
        } else {
            i32::MAX
        };

        let mut best_move = moves[0];

        for mv in moves {
            let mut next_board = board.clone();
            next_board.apply_move(mv, player);

            let (score, _, _) = Self::minimax(&next_board, Self::other_player(player), depth + 1, max_depth);

            if player == Player::X {
                if score > best_score {
                    best_score = score;
                    best_move = mv;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = mv;
                }
            }
        }

        (best_score, best_move.0, best_move.1)
    }
    fn other_player(player: Player) -> Player {
    match player {
        Player::X => Player::O,
        Player::O => Player::X,
    }
}

}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let max_depth = 4;
        Self::minimax(board, player, 0, max_depth)
    }
}