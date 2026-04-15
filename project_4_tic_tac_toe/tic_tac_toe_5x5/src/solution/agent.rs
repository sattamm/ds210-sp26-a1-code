use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();
    let mut total = board.score() * 100;

    fn evaluate_window(
    cells: &Vec<Vec<Cell>>,
    row: isize,
    col: isize,
    dr: isize,
    dc: isize,
) -> i32 {
    let n = cells.len() as isize;

    let r1 = row;
    let c1 = col;
    let r2 = row + dr;
    let c2 = col + dc;
    let r3 = row + 2 * dr;
    let c3 = col + 2 * dc;

    if r1 < 0 || r1 >= n || c1 < 0 || c1 >= n ||
       r2 < 0 || r2 >= n || c2 < 0 || c2 >= n ||
       r3 < 0 || r3 >= n || c3 < 0 || c3 >= n {
        return 0;
    }

    let window = [
        &cells[r1 as usize][c1 as usize],
        &cells[r2 as usize][c2 as usize],
        &cells[r3 as usize][c3 as usize],
    ];

    let mut x_count = 0;
    let mut o_count = 0;
    let mut empty_count = 0;

    for cell in window {
        match cell {
            Cell::X => x_count += 1,
            Cell::O => o_count += 1,
            Cell::Empty => empty_count += 1,
            Cell::Wall => return 0,
        }
    }

    if x_count > 0 && o_count > 0 {
        return 0;
    }

    if x_count == 3 {
        return 100;
    }
    if o_count == 3 {
        return -100;
    }

    if x_count == 2 && empty_count == 1 {
        return 15;
    }
    if o_count == 2 && empty_count == 1 {
        return -15;
    }

    if x_count == 1 && empty_count == 2 {
        return 3;
    }
    if o_count == 1 && empty_count == 2 {
        return -3;
    }

    0
}

    for row in 0..n {
        for col in 0..n {
            total += evaluate_window(&cells, row as isize, col as isize, 0, 1);
            total += evaluate_window(&cells, row as isize, col as isize, 1, 0);
            total += evaluate_window(&cells, row as isize, col as isize, 1, 1);
            total += evaluate_window(&cells, row as isize, col as isize, 1, -1);
        }
    }

    total
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

            let (score, _, _) = Self::minimax(
                &next_board, 
                Self::other_player(player), 
                depth + 1, 
                max_depth
            );

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
    let max_depth = 4;  // ← only change
    Self::minimax(board, player, 0, max_depth)
}
}