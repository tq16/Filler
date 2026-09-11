#[derive(Debug)]
pub struct Turn {
    pub board: Vec<Vec<char>>,
    pub piece: Vec<Vec<char>>,
}

#[derive(Debug)]
pub struct Player {
    pub own_old: char,
    pub own_new: char,
    pub enemy_old: char,
    pub enemy_new: char,
}

impl Player {
    pub fn from_number(number: u8) -> Self {
        if number == 1 {
            Self {
                own_old: '@',
                own_new: 'a',
                enemy_old: '$',
                enemy_new: 's',
            }
        } else {
            Self {
                own_old: '$',
                own_new: 's',
                enemy_old: '@',
                enemy_new: 'a',
            }
        }
    }
}