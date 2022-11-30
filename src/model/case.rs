pub enum Case {
    Player_1,
    Player_2,
    Empty
}

impl ToString for Case {
    fn to_string(&self) -> String {
        match self {
            Case::Player_1 => String::from("X"),
            Case::Player_2 => String::from("O"),
            Case::Empty => String::from("_"),
        }
    }
}