use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
       


       let mut upper_bound = max;
       let mut lower_bound = min;
       let mut mid = (upper_bound + lower_bound) / 2;
       let mut counter = 0;

       while lower_bound <= upper_bound{
        
        let x = player.ask_to_compare(mid);
            if x == -1 {

        upper_bound = mid -1 ;
        mid = (upper_bound + lower_bound) / 2;

        }
        else if x == 1{

        lower_bound = mid + 1 ;
        mid = (upper_bound + lower_bound) / 2;

        }
        else{
        return mid
       }
       }

      return mid;


    }
}
