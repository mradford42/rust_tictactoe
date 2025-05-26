use std::fmt;

/// Struct to hold the 3x3 board state
struct Board {
    board: [char; 9],
}

// List of all possible wins in 3x3 tic tac toe
const WIN_PATTERNS: [(usize, usize, usize); 8] = [
    (0,1,2),
    (3,4,5),
    (6,7,8),
    (0,3,6),
    (1,4,7),
    (2,4,5),
    (0,4,8),
    (2,4,6),
];

// Height and Width of board
const HEIGHT: usize = 3;
const WIDTH: usize = 3;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for (i, space) in self.board.iter().enumerate() {
            string.push_str(format!("| {space} ").as_str());
            if (i+1)%3 == 0 {
                string.push_str("|\n");
            }
        }
        write!(f, "{}", string)
    }
}

impl Board {
    /// Create a new Board object
    fn new() -> Board {
        Board {
            board: [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
        }
    }

    /// Place a symbol at the given 0-based row and column
    /// Returns true if the play is possible, false otherwise
    fn play(&mut self, row: usize, col: usize, symbol: char) -> bool{
        let index = (row*WIDTH) + col;
        if self.board[index] != ' ' { return false;}
        self.board[index] = symbol;
        true
    }

    /// Returns an Option containing the winning char if there is a winner, None otherwise
    fn check_for_win(&self) -> Option<char> {
        for (x,y,z) in WIN_PATTERNS {
            if self.board[x] == self.board[y] && 
                self.board[y] == self.board[z] && 
                self.board[x] != ' '{
                return Some(self.board[x]);
            }
        }
        None
    }

}

/// Struct to hold the game state
pub struct Game {
    board: Board,
    current_player: char,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Create a new Game object
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: 'X',
        }
    }

    /// Play a single turn of the game
    /// Returns true is the game is over, false otherwise
    pub fn play_turn(&mut self, row: usize, col: usize) -> Result<bool, &str> {
        if row >= HEIGHT {
            return Err("0-based row out of bounds of 3x3 board");
        }
        else if col >= WIDTH {
            return Err("0-based column out of bounds of 3x3 board");
        }
        let play_succeeds = self.board.play(row, col, self.current_player);
        if play_succeeds {self.change_player()};
        Ok(play_succeeds)
    }

    pub fn check_for_winner(&self)-> Option<char> {
        self.board.check_for_win()
    }

    /// Change the player from X to O or O to X
    fn change_player(&mut self) {
        match self.current_player {
            'X' => self.current_player = 'O',
            'O' => self.current_player = 'X',
            _ => (), // unreachable since current_player is private
        }
    }

    pub fn current_player(&self) -> char {
        self.current_player
    }

    /// Print the ASCII board to the screen
    pub fn print_board(&self) {
        println!("{}", self.board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win_test() {
        let mut test_board = Board::new();
        test_board.play(0, 0, 'X');
        test_board.play(1, 1, 'X');
        test_board.play(2, 2, 'X');
        assert_eq!(Some('X'), test_board.check_for_win())
    }

    #[test]
    fn non_win_test() {
        let mut test_board = Board::new();
        test_board.play(0, 0, 'X');
        test_board.play(1, 1, 'O');
        test_board.play(2, 2, 'X');
        assert_eq!(None, test_board.check_for_win())
    }

    #[test]
    fn occupied_space() {
        let mut test_board = Board::new();
        let result = test_board.play(0, 0, 'X');
        assert_eq!(result, true);
        let result = test_board.play(0, 0, 'O');
        assert_eq!(result, false);
    }
}
