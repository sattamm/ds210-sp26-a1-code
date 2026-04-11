use std::i32;

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

        //base case, if the game is over then return the board's score
        if board.game_over(){
            return(board.score(), 0, 0); //we don't care about the move, so just return 0,0
        }

        //vector with all legal moves from the current board 
        let possible_moves = board.moves();

        let mut best_score = match player {
            Player::X => i32::MIN, //x wants to max the score
            Player::O => i32::MAX, //o wants to min
        };

        //set a temp best move for now
        let mut best_move = possible_moves[0]; 

        for mv in possible_moves{
            board.apply_move(mv, player); 

            //recurisively solve the resulting board for the other player
            let (child_score, _, _) = SolutionAgent::solve(board, other_player(player), _time_limit);

            //undo the move so the board goes back to the original
            //need this because we want to test the next indepdent move
            board.undo_move(mv, player);

            //update the score based on which player's turn it is
            match player{
                Player::X => {
                    //x wants the move with the largest score
                    if child_score > best_score {
                        best_score = child_score;
                        best_move = mv;
                    }
                } 
                Player::O => {
                    //o wants the move with the smallest score
                    if child_score < best_score {
                        best_score = child_score;
                        best_move = mv;
                    }
                }
            }

        }
        //returns a tuple with three values 
        //the best score, and the best move's row and col
        (best_score, best_move.0, best_move.1)
    }

    //helper function, can help switch the turns

    
}

  fn other_player(player: Player) -> Player {
        match player {
        Player::X => Player::O,
        Player::O => Player::X,
        }
    }
