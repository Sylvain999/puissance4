use std::ops::Add;

use super::case::{Case, Player};

pub struct Board{
    cases : Vec<Vec<Case>>,
    player_turn : u8
}

impl Board {
    pub fn new(width : u32, height : u32) -> Board {

        // we create a matric of empty cases
        let cases = (0..height)
            .map(|_| (
                (0..width)
                    .map(|_| Case::new())
                    .collect::<Vec<Case>>()
            ))
            .collect::<Vec<Vec<Case>>>();

        Board {
            cases : cases,
            player_turn : 0
        }
    }    

    pub fn player_plays(&mut self, coord_x : usize, coord_y : usize) -> Result<u32, String> {
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

        self.prepare_next_turn();

        Ok(0)
    }

    fn prepare_next_turn(&mut self) {
        self.player_turn = (self.player_turn + 1) % 2; 
    } 


}

impl ToString for Board {
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

    use super::Board;

    #[test]
    fn create_empty_board() {
        let board = Board::new(3, 2);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ _ _\n")
        );

    }

    #[test]
    fn player_1_plays_in_bottom_right() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(0, 2).is_ok(), true);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ _ X\n")
        );
    }

    #[test]
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
    fn out_of_bound() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(3, 2).is_ok(), false);

        assert_eq!(board.player_plays(0, 3).is_ok(), false) ;

        assert_eq!(board.player_plays(3, 3).is_ok(), false) ;
    }

    #[test]
    fn player_cant_play_if_already_played() {
        let mut board = Board::new(3, 2);

        assert_eq!(board.player_plays(0, 0).is_ok(), true);

        assert_eq!(board.player_plays(0, 0).is_ok(), false);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("X _ _\n")
        );
    }
}