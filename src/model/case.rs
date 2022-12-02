use std::{u8};

#[derive(Debug)]
pub enum Player {
    Player1,
    Player2
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Player {
    pub fn eq_index(&self, index : u8) -> bool {
        match index {
            0 => *self == Player::Player1,
            1 => *self == Player::Player2,
            _default => false,
        }
    }
}


pub struct Case {
    player : Option<Player>,
}

impl Case {
    pub fn new() -> Case {
        Case {
            player : None,
        }
    }
    
    pub fn set_player(&mut self, player : Player) -> Result<(), &str> {
        if let Some(_) = self.player
        {
            Err("A player has already played in this case")
        } else {
            self.player = Some(player);
            Ok(())
        }        
    }

    pub fn get_player(&self) -> &Option<Player> {
        &self.player
    }
}

impl ToString for Case {
    fn to_string(&self) -> String {
        if let Some(player) = &self.player {
            match player {
                Player::Player1 => String::from("X"),
                Player::Player2 => String::from("O"),
            }
        } else {
            String::from("_")
        }
    }
}