pub enum Player {
    Player1,
    Player2
}


pub struct Case {
    player : Option<Player>
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