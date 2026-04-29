use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
    if board.game_over() {
        //base case, the game is already over
        //board.score() says whoever is winning 
        return (board.score(), 0, 0);
    }

    //all the current legal moves available 
    let possible_moves = board.moves();

    //store the best move, 0,0 is a filler for now
    let mut best_move = (0, 0);

    //X wants to max the score, O wants to min the score 
    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    //try every possible move 
    for x in possible_moves {

        //clones the board so we can test a move without changing current
        let mut new_board = board.clone();

        //apply move 
        new_board.apply_move(x, player);

        //switch turns
        let next_player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        //recurisvely solve the board from the next player's turn
        //we only care about the score technically, not the returned moves
        let (score, _, _) = SolutionAgent::solve(&mut new_board, next_player, _time_limit);

        //find the best score 
        match player {
            Player::X => {
                if score > best_score {
                    best_score = score;
                    best_move = x;
                }
            }
            Player::O => {
                if score < best_score {
                    best_score = score;
                    best_move = x;
                }
            }
        }
    }
    //return the best score and the best move row and col
    (best_score, best_move.0, best_move.1)

    }
}
