extern crate core;

use std::cmp;
use std::fmt;
use std::io::BufRead;
use std::process::exit;

trait MinimaxGame {
    fn computer_move(&mut self);
    fn evaluate(&self) -> i32;
    fn minimax(&mut self, depth: i32, is_max: bool) -> i32;
}

const BOARD_UTF8_SYMBOLS_IN_ROW: u8 = 13;
const BOARD_ROWS: u8 = 7;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    X,
    O,
    Free,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Player,
    Computer,
}

#[derive(Debug, PartialEq)]
enum GameState {
    Win(Move),
    Draw,
}

struct Board {
    field: Vec<Vec<Tile>>,
    current_move: Move,
    computer_tile: Tile,
    player_tile: Tile,
}

impl MinimaxGame for Board {
    fn computer_move(&mut self) {
        if !self.has_free_tiles() {
            panic!("No free tiles!");
        }

        let mut best_val = -1000;
        let mut best_move = (0, 0);

        for i in 0..3 {
            for j in 0..3 {
                if self.field[i][j] == Tile::Free {
                    self.field[i][j] = self.computer_tile;

                    let move_val = self.minimax(0, false);

                    self.field[i][j] = Tile::Free;

                    if move_val > best_val {
                        best_move = (i, j);
                        best_val = move_val;
                    }
                }
            }
        }

        self.make_move(best_move, self.computer_tile)
    }
    fn evaluate(&self) -> i32 {
        match self.analyse() {
            Some(GameState::Win(Move::Computer)) => 10,
            Some(GameState::Win(Move::Player)) => -10,
            _ => 0,
        }
    }
    fn minimax(&mut self, depth: i32, is_max: bool) -> i32 {
        let score = self.evaluate();

        if score == 10 {
            return score - depth;
        }

        if score == -10 {
            return score + depth;
        }

        if !self.has_free_tiles() {
            return 0;
        }

        if is_max {
            let mut best = -1000;

            for i in 0..3 {
                for j in 0..3 {
                    if self.field[i][j] == Tile::Free {
                        self.field[i][j] = self.computer_tile;
                        self.change_player();

                        best = cmp::max(best, self.minimax(depth + 1, !is_max));

                        self.field[i][j] = Tile::Free;
                        self.change_player();
                    }
                }
            }

            best
        } else {
            let mut best = 1000;

            for i in 0..3 {
                for j in 0..3 {
                    if self.field[i][j] == Tile::Free {
                        self.field[i][j] = self.player_tile;
                        self.change_player();

                        best = cmp::min(best, self.minimax(depth + 1, !is_max));

                        self.field[i][j] = Tile::Free;
                        self.change_player();
                    }
                }
            }

            best
        }
    }
}

impl Board {
    fn analyse(&self) -> Option<GameState> {
        for row in 0..3 {
            if self.field[row][0] == self.field[row][1]
                && self.field[row][1] == self.field[row][2]
                && self.field[row][0] != Tile::Free
            {
                return Some(GameState::Win(self.current_move));
            }
        }

        for col in 0..3 {
            if self.field[0][col] == self.field[1][col]
                && self.field[1][col] == self.field[2][col]
                && self.field[0][col] != Tile::Free
            {
                return Some(GameState::Win(self.current_move));
            }
        }

        if self.field[0][0] == self.field[1][1]
            && self.field[1][1] == self.field[2][2]
            && self.field[0][0] != Tile::Free
        {
            return Some(GameState::Win(self.current_move));
        }

        if self.field[0][2] == self.field[1][1]
            && self.field[1][1] == self.field[2][0]
            && self.field[0][2] != Tile::Free
        {
            return Some(GameState::Win(self.current_move));
        }

        if self.has_free_tiles() {
            None
        } else {
            Some(GameState::Draw)
        }
    }
    fn check_move(&self, (row, col): (usize, usize)) -> Result<(usize, usize), &str> {
        if row <= 2 && col <= 2 {
            if self.field[row][col] == Tile::Free {
                Ok((row, col))
            } else {
                Err("Choose free tile!")
            }
        } else {
            Err("Place tile in bounds (0 <= col <= 2, 0 <= row <= 2)!")
        }
    }
    fn make_move(&mut self, (row, col): (usize, usize), tile: Tile) {
        self.field[row][col] = tile;
    }

    fn has_free_tiles(&self) -> bool {
        self.field.iter().any(|row| row.contains(&Tile::Free))
    }
    fn change_player(&mut self) {
        self.current_move = match self.current_move {
            Move::Player => Move::Computer,
            Move::Computer => Move::Player,
        }
    }

    fn computer_player_move(&mut self) {
        if !self.has_free_tiles() {
            panic!("No free tiles!");
        }
        let mut best_val = -1000;
        let mut best_move = (0, 0);

        for i in 0..3 {
            for j in 0..3 {
                if self.field[i][j] == Tile::Free {
                    self.field[i][j] = self.player_tile;

                    let move_val = self.minimax_player(0, false);

                    self.field[i][j] = Tile::Free;

                    if move_val > best_val {
                        best_move = (i, j);
                        best_val = move_val;
                    }
                }
            }
        }

        self.make_move(best_move, self.player_tile)
    }
    fn evaluate_player(&self) -> i32 {
        match self.analyse() {
            Some(GameState::Win(Move::Player)) => 10,
            Some(GameState::Win(Move::Computer)) => -10,
            _ => 0,
        }
    }
    fn minimax_player(&mut self, depth: i32, is_max: bool) -> i32 {
        let score = self.evaluate_player();

        if score == 10 {
            return score - depth;
        }

        if score == -10 {
            return score + depth;
        }

        if !self.has_free_tiles() {
            return 0;
        }

        if is_max {
            let mut best = -1000;

            for i in 0..3 {
                for j in 0..3 {
                    if self.field[i][j] == Tile::Free {
                        self.field[i][j] = self.player_tile;
                        self.change_player();

                        best = cmp::max(best, self.minimax_player(depth + 1, !is_max));

                        self.field[i][j] = Tile::Free;
                        self.change_player();
                    }
                }
            }

            best
        } else {
            let mut best = 1000;

            for i in 0..3 {
                for j in 0..3 {
                    if self.field[i][j] == Tile::Free {
                        self.field[i][j] = self.computer_tile;
                        self.change_player();

                        best = cmp::min(best, self.minimax_player(depth + 1, !is_max));

                        self.field[i][j] = Tile::Free;
                        self.change_player();
                    }
                }
            }

            best
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr: String = String::new();
        repr.reserve((BOARD_UTF8_SYMBOLS_IN_ROW * BOARD_ROWS) as usize);

        for i in 0..self.field.len() {
            if i == 0 {
                repr.push_str(
                    format!(
                        "-------------\n\
                        | {} | {} | {} |\n\
                        -------------\n",
                        self.field[i][0], self.field[i][1], self.field[i][2]
                    )
                    .as_str(),
                )
            } else {
                repr.push_str(
                    format!(
                        "| {} | {} | {} |\n\
                        -------------\n",
                        self.field[i][0], self.field[i][1], self.field[i][2]
                    )
                    .as_str(),
                )
            }
        }

        write!(f, "{}", repr).expect("Failed to represent Board");

        Ok(())
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tile::X => write!(f, "X"),
            Tile::O => write!(f, "O"),
            Tile::Free => write!(f, " "),
        }
    }
}

fn parse_first_move(s: &str) -> Result<Move, &str> {
    match s.trim() {
        "p" | "P" => Ok(Move::Player),
        "c" | "C" => Ok(Move::Computer),
        _ => Err("Please enter correct participant!"),
    }
}

fn parse_pos(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split(',');
    let row = parts.next()?.trim().parse().ok()?;
    let col = parts.next()?.trim().parse().ok()?;
    Some((row, col))
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    println!("Computer - C, Player - P");
    println!("Enter who will play first: ");
    let first_move = loop {
        let mut first_player_line = String::new();
        stdin.read_line(&mut first_player_line).unwrap();
        match parse_first_move(&first_player_line) {
            Ok(first_move) => {
                break first_move;
            }
            Err(error) => {
                println!("{}", error)
            }
        };
    };

    let mut board = Board {
        field: vec![vec![Tile::Free; 3]; 3],
        current_move: first_move,
        computer_tile: Tile::X,
        player_tile: Tile::O,
    };

    println!("Current board configuration:");
    println!("{}", board);

    let game_result: GameState = loop {
        println!("{:?}'s move: ", board.current_move);
        match board.current_move {
            Move::Player => {
                let (row, col) = loop {
                    let mut line = String::new();
                    stdin.read_line(&mut line).unwrap();

                    let (row, col) = match parse_pos(&line) {
                        Some((row, col)) => (row, col),
                        None => {
                            println!("Please enter correct move!");
                            continue;
                        }
                    };

                    match board.check_move((row, col)) {
                        Ok(mv) => break mv,
                        Err(err) => {
                            println!("{}", err);
                            continue;
                        }
                    }
                };
                board.make_move((row, col), board.player_tile);
            }
            Move::Computer => {
                board.computer_move();
            }
        }

        println!("Current board configuration:");
        println!("{}", board);

        break match board.analyse() {
            Some(GameState::Win(Move::Computer)) => GameState::Win(Move::Computer),
            Some(GameState::Win(Move::Player)) => GameState::Win(Move::Player),
            Some(GameState::Draw) => GameState::Draw,
            None => {
                board.change_player();
                continue;
            }
        };
    };

    match game_result {
        GameState::Draw => {
            println!("Draw!");
            exit(0)
        }
        _ => {
            println!("{:?} won!", board.current_move)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, GameState, MinimaxGame, Move, Tile};

    #[test]
    fn test_win_move() {
        let mut board = Board {
            field: vec![
                vec![Tile::Free, Tile::X, Tile::X],
                vec![Tile::X, Tile::O, Tile::O],
                vec![Tile::O, Tile::O, Tile::Free],
            ],
            current_move: Move::Computer,
            computer_tile: Tile::X,
            player_tile: Tile::O,
        };
        board.computer_move();
        println!("{}", board);
        assert!(board.field[0][0] == board.computer_tile);
    }

    #[test]
    fn test_block_move() {
        let mut board = Board {
            field: vec![
                vec![Tile::O, Tile::Free, Tile::X],
                vec![Tile::Free, Tile::X, Tile::O],
                vec![Tile::O, Tile::O, Tile::X],
            ],
            current_move: Move::Computer,
            computer_tile: Tile::X,
            player_tile: Tile::O,
        };
        board.computer_move();
        println!("{}", board);
        assert!(board.field[1][0] == board.computer_tile);
    }

    #[test]
    fn pc_vs_pc_1move_by_player() {
        let first_moves = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut results: Vec<GameState> = vec![];
        for first_move in first_moves {
            let row = first_move / 3;
            let col = first_move % 3;
            let mut board = Board {
                field: vec![
                    vec![Tile::Free, Tile::Free, Tile::Free],
                    vec![Tile::Free, Tile::Free, Tile::Free],
                    vec![Tile::Free, Tile::Free, Tile::Free],
                ],
                current_move: Move::Computer,
                computer_tile: Tile::X,
                player_tile: Tile::O,
            };
            board.field[row][col] = board.player_tile;
            for _ in 0..8 {
                match board.current_move {
                    Move::Player => {
                        board.computer_player_move();
                    }
                    Move::Computer => {
                        board.computer_move();
                    }
                }
                board.change_player();
            }

            results.push(board.analyse().unwrap());
        }

        assert!(results.iter().any(|r| *r == GameState::Draw));
    }

    #[test]
    fn pc_vs_pc_1move_by_computer() {
        let first_moves = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut results: Vec<GameState> = vec![];
        for first_move in first_moves {
            let row = first_move / 3;
            let col = first_move % 3;
            let mut board = Board {
                field: vec![
                    vec![Tile::Free, Tile::Free, Tile::Free],
                    vec![Tile::Free, Tile::Free, Tile::Free],
                    vec![Tile::Free, Tile::Free, Tile::Free],
                ],
                current_move: Move::Player,
                computer_tile: Tile::X,
                player_tile: Tile::O,
            };
            board.field[row][col] = board.computer_tile;
            for _ in 0..8 {
                match board.current_move {
                    Move::Player => {
                        board.computer_player_move();
                    }
                    Move::Computer => {
                        board.computer_move();
                    }
                }
                board.change_player();
            }

            results.push(board.analyse().unwrap());
        }

        assert!(results.iter().any(|r| *r == GameState::Draw));
    }
}
