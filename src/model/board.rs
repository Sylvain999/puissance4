use std::ops::Add;

use super::case::{Case, Player};

pub struct Board{
    cases : Vec<Vec<Case>>,
    player_turn : u8,
    winner : Option<Player>,
}

impl Board {

    /// Create a new empty board, where nobody has played
    /// 
    ///  # Example
    /// 
    /// ```
    /// let board = Board::new(3, 4);
    /// ```
    pub fn new(width : u32, height : u32) -> Board {

        // we create a matric of empty cases
        let cases = (0..height)
            .map(|y| (
                (0..width)
                    .map(|x| Case::new())
                    .collect::<Vec<Case>>()
            ))
            .collect::<Vec<Vec<Case>>>();

        Board {
            cases : cases,
            player_turn : 0,
            winner : None,
        }
    }    

    /// The player whose turn it is can play in the board
    /// Can return an Error if it is out of bound, or if the case has already been played
    /// 
    /// # Example
    /// 
    /// ```
    /// let board = Board::new(4, 4);
    /// assert_eq!(board.player_plays(0,2).is_ok(), true);
    /// 
    /// assert_eq!(board.player_plays(0,4).is_ok(), false);
    /// 
    /// assert_eq!(board.player_plays(0,2).is_ok(), false);
    /// ```
    pub fn player_plays(&mut self, coord_x : usize, coord_y : usize) -> Result<(), String> {
        if self.winner.is_some() {
            return Err(String::from("Can't play a game that is finished"));
        }

        self.cases.get_mut(coord_x)
            .ok_or("X coordinates out of bound")?
            .get_mut(coord_y)
            .ok_or("Y coordinates out of bound")?
            .set_player(
                if self.player_turn == 0 {
                    Player::Player1
                } else {
                    Player::Player2
                }
            )?;

        self.computes_winner(coord_x, coord_y);

        self.prepare_next_turn();

        Ok(())
    }

    /// Prepare the next turn to have the next player to play
    /// 
    /// # Example
    /// 
    /// ```
    /// let board = Board::new(4, 4); // it is the turn of player 1
    /// 
    /// board.prepare_next_turn(); // it is the turn of player 2
    /// 
    /// board.prepare_next_turn(); // it is the turn of player 1 again
    /// ```
    fn prepare_next_turn(&mut self) {
        self.player_turn = (self.player_turn + 1) % 2; 
    }

    pub fn get_cases(&self) -> &Vec<Vec<Case>> {
        &self.cases
    }

    /// Give the strike, series number of different cases that are close to each other, and make a line (column = false, row =true), 
    /// a column (column = true, row =false) or a diagonal (column = true, row =true)
    fn get_strike(&self, coord_x : usize, coord_y : usize, column : bool, row : bool) -> u32 {
        let mut strike : u32 = 0;
        
        let mut tmp_coord_x : i32 = coord_x as i32;
        let mut tmp_coord_y : i32 = coord_y as i32;

        // we check what's next
        while tmp_coord_x < self.cases.len() as i32 && tmp_coord_y < self.cases.get(0).unwrap().len() as i32 {

            if let Some(case_value) = self.cases.get(tmp_coord_x as usize).unwrap().get(tmp_coord_y as usize){
                if let Some(player_value) = case_value.get_player() {
                    if player_value.eq_index(self.player_turn) {
    
                        strike += 1;
                        tmp_coord_x += row as i32;
                        tmp_coord_y += column as i32;
    
                    } else {
                        break;
                    }
                } else {
                    break;
                }

            } else {
                break;
            }
        }

        // we check what's before
        tmp_coord_x = coord_x as i32 - row as i32;
        tmp_coord_y = coord_y as i32 - column as i32;
        
        while tmp_coord_x >= 0 && tmp_coord_y >= 0 {
            if let Some(case_value) = self.cases.get(tmp_coord_x as usize).unwrap().get(tmp_coord_y as usize){
                
                if let Some(player_value) = case_value.get_player() {
                    if player_value.eq_index(self.player_turn) {
    
                        strike += 1;
                        tmp_coord_x -= row as i32;
                        tmp_coord_y -= column as i32;
    
                    } else {
                        break;
                    }
                } else {
                    break;
                }

            } else {
                break;
            }
        }

        strike
    }

    /// Check if there is a winner that made a column
    fn check_winner_column(&self, coord_x : usize, coord_y : usize) -> bool {
        self.get_strike(coord_x, coord_y, true, false) >= 4
    }

    /// Check if there is a winner that made a column
    fn check_winner_row(&self, coord_x : usize, coord_y : usize) -> bool {
        self.get_strike(coord_x, coord_y, false, true) >= 4
    }

    /// Check if there is a winner that made a diagonal
    fn check_winner_diagonal(&self, coord_x : usize, coord_y : usize) -> bool {
        self.get_strike(coord_x, coord_y, true, true) >= 4
    }

    /// Tell the winner if there is one
    fn computes_winner(&mut self, coord_x : usize, coord_y : usize) {
        
        if self.check_winner_column(coord_x, coord_y) 
             || self.check_winner_row(coord_x, coord_y)
             || self.check_winner_diagonal(coord_x, coord_y) {

            self.winner =  match self.player_turn {
                0 => Some(Player::Player1),
                1 => Some(Player::Player2),
                default => None,
            };

        }

    }

    /// Tell the winner of the game (None if there is none)
    pub fn get_winner(&self) -> &Option<Player> {
        &self.winner
    }


}

impl ToString for Board {

    /// Display the Board so one can understand the situation of the board
    /// 
    /// # Example
    /// 
    /// ```
    /// let board = Board::new(4, 4);
    /// 
    /// board.player_plays(0,2);
    /// board.player_plays(0,1);
    /// 
    /// assert_eq!(board.to_string(), 
    ///     String::from("_ _ _\n")
    ///     .add("_ O X\n")
    /// );
    /// 
    /// ```
    fn to_string(&self) -> String {
        self.cases.iter()
            .map(|liste| {
                liste.iter()
                    .map(|case| case.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
                    .add("\n")
                }
            )
            .rev()
            .collect::<Vec<String>>()
            .concat()
    }
}



#[cfg(test)]
mod tests {
    use std::ops::Add;

    use crate::model::case::Player;

    use super::Board;
    
    #[test]
    /// Create an empty board and verifies it has been well initialised
    fn create_empty_board() {
        let board = Board::new(3, 2);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ _ _\n")
        );

    }

    #[test]
    /// Creates a board, and make one play on the bottom right. Verifies that the play was done
    fn player_1_plays_in_bottom_right() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(0, 2).is_ok(), true);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ _ X\n")
        );
    }

    #[test]
    /// Make two player have a play. Verifies that both were done, and that it is two different players
    fn both_player_play() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(0, 2).is_ok(), true);
        assert_eq!(board.player_plays(0, 1).is_ok(), true);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ O X\n")
        );
    }

    #[test]
    /// Make players play out of bound
    fn out_of_bound() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(3, 2).is_ok(), false);

        assert_eq!(board.player_plays(0, 3).is_ok(), false) ;

        assert_eq!(board.player_plays(3, 3).is_ok(), false) ;
    }

    #[test]
    /// Make player 1 play, and player 2 play on the same place, Error
    fn player_cant_play_if_already_played() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(0, 0).is_ok(), true);

        assert_eq!(board.player_plays(0, 0).is_ok(), false);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("X _ _\n")
        );
    }

    #[test]
    /// We see if a player has made a column
    fn has_winner_with_column() {
        let mut board = Board::new(4, 10);

        // Turn 1 
        assert_eq!(board.player_plays(0, 0).is_ok(), true);
        assert_eq!(board.player_plays(1, 0).is_ok(), true);

        // Turn 12
        assert_eq!(board.player_plays(0, 1).is_ok(), true);
        assert_eq!(board.player_plays(1, 1).is_ok(), true);

        // Turn 3
        assert_eq!(board.player_plays(0, 2).is_ok(), true);
        assert_eq!(board.player_plays(1, 2).is_ok(), true);

        // Turn 4 
        assert_eq!(board.player_plays(0, 3).is_ok(), true);

        assert_eq!(*board.get_winner(), Some(Player::Player1));


    }

    #[test]
    /// We see if a player has made a row
    fn has_winner_with_row() {
        let mut board = Board::new(4, 10);

        // Turn 1 
        assert_eq!(board.player_plays(0, 0).is_ok(), true);
        assert_eq!(board.player_plays(0, 1).is_ok(), true);

        // Turn 12
        assert_eq!(board.player_plays(1, 0).is_ok(), true);
        assert_eq!(board.player_plays(1, 1).is_ok(), true);

        // Turn 3
        assert_eq!(board.player_plays(2, 0).is_ok(), true);
        assert_eq!(board.player_plays(2, 1).is_ok(), true);

        // Turn 4 
        assert_eq!(board.player_plays(3, 0).is_ok(), true);

        assert_eq!(*board.get_winner(), Some(Player::Player1));
    }

    #[test]
    /// We see if a player has made a diagonale
    fn has_winner_with_diagonale() {
        let mut board = Board::new(4, 10);

        // Turn 1 
        assert_eq!(board.player_plays(0, 0).is_ok(), true);
        assert_eq!(board.player_plays(0, 1).is_ok(), true);

        // Turn 12
        assert_eq!(board.player_plays(1, 1).is_ok(), true);
        assert_eq!(board.player_plays(0, 2).is_ok(), true);

        // Turn 3
        assert_eq!(board.player_plays(2, 2).is_ok(), true);
        assert_eq!(board.player_plays(0, 3).is_ok(), true);

        // Turn 4 
        assert_eq!(board.player_plays(3, 3).is_ok(), true);

        assert_eq!(*board.get_winner(), Some(Player::Player1));


    }

    #[test]
    // We cannot continue the game if there is already a winner
    fn does_not_continue_game_if_winner() {
        let mut board = Board::new(4, 4);

        // Turn 1 
        assert_eq!(board.player_plays(0, 0).is_ok(), true);
        assert_eq!(board.player_plays(0, 1).is_ok(), true);

        // Turn 12
        assert_eq!(board.player_plays(1, 0).is_ok(), true);
        assert_eq!(board.player_plays(1, 1).is_ok(), true);

        // Turn 3
        assert_eq!(board.player_plays(2, 0).is_ok(), true);
        assert_eq!(board.player_plays(2, 1).is_ok(), true);

        // Turn 4 
        assert_eq!(board.player_plays(3, 0).is_ok(), true);
        assert_eq!(board.player_plays(3, 1).is_ok(), false);

        assert_eq!(*board.get_winner(), Some(Player::Player1));

        assert_eq!(board.to_string(), 
            String::from("X _ _ _\n")
                    .add("X O _ _\n")
                    .add("X O _ _\n")
                    .add("X O _ _\n")
        );



    }

}