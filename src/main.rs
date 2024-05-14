use std::io;

const RESET: &str = "\x1b[0m";
const ORANGE: &str = "\x1b[93m";
const RED: &str = "\x1b[0;31m";

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

type Board = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Player {
    One = 1,
    Two = 2,
    None = 0,
}

impl Player {
    fn from_int(int: u8) -> Player {
        match int {
            1 => Player::One,
            2 => Player::Two,
            _ => Player::None,
        }
    }
}

#[derive(Debug)]
enum MoveError {
    GameFinished,
    InvalidColumn,
    ColumnFull,
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::ColumnFull => write!(f, "column is full"),
            MoveError::InvalidColumn => write!(f, "column must be between 1 and 7"),
            MoveError::GameFinished => write!(f, "game is already finished"),
        }
    }
}

struct Game {
    current_move: u8,
    current_player: Player,
    board: Board,
    is_finished: bool,
    winner: Player,
}

impl Game {
    fn default() -> Game {
        Game {
            current_move: 0,
            current_player: Player::One,
            board: [
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0],
            ],
            is_finished: false,
            winner: Player::None,
        }
    }

    fn clear_screen(&self) {
        print!("{}[2J", 27 as char);
    }

    fn display_board(&self) {
        self.clear_screen();

        println!("{}--------------------{}", ORANGE, RESET);
        println!("{}CONNECT 4 (Move {}){}", ORANGE, self.current_move, RESET);
        println!("{}--------------------{}", ORANGE, RESET);

        for row in self.board {
            let row_str: String = row
                .iter()
                .map(|&cell| match cell {
                    1 => "🔴",
                    2 => "🟡",
                    _ => "⚫",
                })
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}", row_str);
        }

        println!("{}--------------------{}", ORANGE, RESET);

        if self.is_finished {
            match self.winner {
                Player::One => println!("{}🔴 Player 1 has won!{}", ORANGE, RESET),
                Player::Two => println!("{}🟡 Player 2 has won!{}", ORANGE, RESET),
                Player::None => println!("{}It's a draw!{}", ORANGE, RESET),
            }

            println!("{}--------------------{}", ORANGE, RESET);
        }
    }

    fn display_error(&self, error: String) {
        self.display_board();
        println!("{}Error: {}{}", RED, error, RESET);
    }

    fn calculate_winner(&mut self) -> Player {
        if self.current_move < BOARD_WIDTH as u8 {
            return Player::None;
        }

        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let cell = self.board[row][col];

                if cell != 0 {
                    let directions = [
                        (0, 1),  // horizontal
                        (1, 0),  // vertical
                        (1, 1),  // diagonal (top-left to bottom-right)
                        (-1, 1), // diagonal (bottom-left to top-right)
                    ];

                    for (row_step, col_step) in directions {
                        let mut consecutive_count = 1;
                        let mut r = row as isize + row_step;
                        let mut c = col as isize + col_step;

                        while r >= 0
                            && r < BOARD_HEIGHT as isize
                            && c >= 0
                            && c < BOARD_WIDTH as isize
                        {
                            if self.board[r as usize][c as usize] == cell {
                                consecutive_count += 1;

                                if consecutive_count == 4 {
                                    self.is_finished = true;
                                    return Player::from_int(cell);
                                }
                            } else {
                                break;
                            }
                            r += row_step;
                            c += col_step;
                        }
                    }
                }
            }
        }

        if self.current_move >= BOARD_HEIGHT as u8 * BOARD_WIDTH as u8 {
            self.is_finished = true;
        }

        Player::None
    }

    fn play_move(&mut self, column: usize) -> Result<(), MoveError> {
        if self.is_finished {
            return Err(MoveError::GameFinished);
        }

        if column >= BOARD_WIDTH {
            return Err(MoveError::InvalidColumn);
        }

        if let Some(row) = (0..BOARD_HEIGHT)
            .rev()
            .find(|&row| self.board[row][column] == 0)
        {
            self.board[row][column] = self.current_player as u8;
            self.current_move += 1;
        } else {
            return Err(MoveError::ColumnFull);
        }

        let calculated_winner = self.calculate_winner();

        if calculated_winner != Player::None {
            self.winner = calculated_winner;
        } else {
            self.current_player = match self.current_player {
                Player::One => Player::Two,
                _ => Player::One,
            };
        }

        Ok(())
    }
}

fn main() {
    let mut game = Game::default();
    game.display_board();

    loop {
        while !game.is_finished {
            println!("\n");

            match game.current_player {
                Player::One => println!("PLAYER 1"),
                Player::Two => println!("PLAYER 2"),
                _ => (),
            };

            println!("Enter a column between 1 and 7:");

            let mut user_move = String::new();
            io::stdin()
                .read_line(&mut user_move)
                .expect("Failed to read line");

            let user_move: usize = match user_move.trim().parse() {
                Ok(num) => {
                    if num < 1 || num > 7 {
                        game.display_error(MoveError::InvalidColumn.to_string());
                        continue;
                    } else {
                        num
                    }
                }
                Err(err) => {
                    game.display_error(err.to_string());
                    continue;
                }
            };

            match game.play_move(user_move - 1) {
                Ok(_) => {
                    game.display_board();
                }
                Err(err) => {
                    game.display_error(err.to_string());
                }
            }
        }

        println!("Press 'R' to restart or 'Q' to quit the game.");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim() {
            "R" | "r" => {
                game = Game::default();
                game.display_board();
            }
            "Q" | "q" => {
                println!("Quitting...");
                break;
            }
            _ => game.display_error("invalid input".to_string()),
        }
    }
}
