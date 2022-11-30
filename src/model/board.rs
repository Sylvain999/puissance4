use std::ops::Add;

use super::case::Case;

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
                    .map(|_| Case::Empty)
                    .collect::<Vec<Case>>()
            ))
            .collect::<Vec<Vec<Case>>>();

        Board {
            cases : cases,
            player_turn : 0
        }
    }    

    pub fn player_plays(&mut self, coord_x : usize, coord_y : usize) {
        if self.player_turn == 0 {
            self.cases[coord_x][coord_y] = Case::Player_1;
            self.player_turn = 1
        } else {
            self.cases[coord_x][coord_y] = Case::Player_2;
            self.player_turn = 0
        }
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
        board.player_plays(0, 2);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ _ X\n")
        );
    }

    #[test]
    fn both_player_play() {
        let mut board = Board::new(3, 2);
        board.player_plays(0, 2);
        board.player_plays(0, 1);

        assert_eq!(board.to_string(), 
            String::from("_ _ _\n")
                    .add("_ O X\n")
        );
    }
}