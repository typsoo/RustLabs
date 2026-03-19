use std::io;

struct TicTacToe {
    board: [char; 9],
    current_player: char,
    turns_played: u8,
}

impl TicTacToe {
    const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    fn new(first_player: char) -> Self {
        Self {
            board: [' '; 9],
            current_player: first_player,
            turns_played: 0,
        }
    }

    fn print_board(&self) {
        println!("\nBoard:");
        for (i, row) in self.board.chunks(3).enumerate() {
            println!(" {} | {} | {} ", row[0], row[1], row[2]);
            if i < 2 {
                println!("___|___|___");
            }
        }
        println!();
    }

    fn make_a_move(&mut self) {
        loop {
            println!("Player '{}', type a number (0-8):", self.current_player);

            let Some(digit) = get_user_char().to_digit(10) else {
                println!("Invalid input! Please enter a number.");
                continue;
            };

            let cell_index = digit as usize;

            if self.board[cell_index] != ' ' {
                println!("This cell is already taken!");
                continue;
            }

            self.board[cell_index] = self.current_player;
            self.turns_played += 1;
            break;
        }
    }

    fn check_winner(&self) -> Option<char> {
        for &[a, b, c] in &TicTacToe::WINNING_COMBINATIONS {
            if self.board[a] != ' '
                && self.board[a] == self.board[b]
                && self.board[a] == self.board[c]
            {
                return Some(self.board[a]);
            }
        }

        return None;
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
    }
}

fn get_user_char() -> char {
    let mut user_input = String::new();
    loop {
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if let Some(c) = user_input.trim().chars().next() {
            return c.to_ascii_uppercase();
        }
    }
}

fn choose_first_player() -> char {
    loop {
        println!("Choose who goes first (X or O):");
        let choice = get_user_char();
        if choice == 'X' || choice == 'O' {
            return choice;
        }
        println!("Please type exactly 'X' or 'O'.");
    }
}

fn main() {
    let first_player = choose_first_player();
    let mut game = TicTacToe::new(first_player);

    game.print_board();

    loop {
        game.make_a_move();
        game.print_board();

        if let Some(winner) = game.check_winner() {
            println!("Game over! Player '{}' has won! Congratulations!", winner);
            break;
        } else if game.turns_played == 9 {
            println!("Game over! It's a tie!");
            break;
        }

        game.switch_player();
    }
}
