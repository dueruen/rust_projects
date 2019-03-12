use std::fmt;
use std::io;

fn main() {
    let mut game = Game::new();
    game.run();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Game {
    game_board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            game_board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }

    fn run(&mut self) {
        loop {
            println!("Current player is: {:?}", self.current_player);
            self.draw_game();
            let input = self.get_user_input();
            let tuple = match input {
                Ok(n) => n,
                Err(e) => {
                    println!("Wrong input: {:?}", e);
                    continue;
                }
            };
            if !self.update(tuple.0, tuple.1) {
                println!("Try again!!");
                continue;
            }
            if self.is_there_a_winner() {
                self.draw_game();
                println!("The winner is: {:?}", self.current_player);
                break;
            }
            if self.is_draw() {
                self.draw_game();
                println!("Game over, you are draw!!");
                break;
            }
            self.change_player();
        }
    }

    fn is_draw(&self) -> bool {
        let is_draw = true;
        for i in 0..self.game_board.len() {
            for j in 0..self.game_board[i].len() {
                if self.game_board[i][j] == None {
                    return false;
                }
            }
        }
        is_draw
    }

    fn is_there_a_winner(&self) -> bool {
        for row in 0..self.game_board.len() {
            if self.game_board[0][row] != None
                && self.game_board[0][row] == self.game_board[1][row]
                && self.game_board[1][row] == self.game_board[2][row]
            {
                return true;
            }
        }
        for col in 0..self.game_board.len() {
            if self.game_board[col][0] != None
                && self.game_board[col][0] == self.game_board[col][1]
                && self.game_board[col][1] == self.game_board[col][2]
            {
                return true;
            }
        }
        if self.game_board[0][0] != None
            && self.game_board[0][0] == self.game_board[1][1]
            && self.game_board[1][1] == self.game_board[2][2]
        {
            return true;
        }
        if self.game_board[2][0] != None
            && self.game_board[2][0] == self.game_board[1][1]
            && self.game_board[1][1] == self.game_board[0][2]
        {
            return true;
        }

        false
    }

    fn update(&mut self, x: usize, y: usize) -> bool {
        if self.game_board[x][y] != None {
            return false;
        }
        self.game_board[x][y] = Some(self.current_player);
        true
    }

    fn change_player(&mut self) {
        if self.current_player == Player::X {
            self.current_player = Player::O;
        } else {
            self.current_player = Player::X;
        }
    }

    fn draw_game(&self) {
        for i in 0..3 {
            for j in 0..3 {
                match self.game_board[i][j] {
                    Some(s) => print!("| {:?} ", s),
                    None => print!("|  "),
                };
                ;
            }
            println!("|");
        }
        println!("Give input, (1 1):")
    }

    fn get_user_input(&self) -> Result<(usize, usize), (&str)> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let split: Vec<&str> = input.trim().split(' ').collect();

        if split.len() < 2 {
            return Err("Not enough arguments!! Try e.g: 1 1");
        }

        let x = match split[0].parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err("Not a number between 0 - 2"),
        };

        let y = match split[1].parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err("Not a number between 0 - 2"),
        };

        Ok((x, y))
    }
}
