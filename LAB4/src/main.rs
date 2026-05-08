#[derive(Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
struct Chessman {
    piece_type: PieceType,
    color: Color,
    position: Position,
}

impl Chessman {
    fn move_to(&mut self, new_position: Position) -> bool {
        let dx = self.position.x.abs_diff(new_position.x);
        let dy = self.position.y.abs_diff(new_position.y);

        if dx == 0 && dy == 0 {
            return false;
        }

        let is_valid = match self.piece_type {
            PieceType::Pawn => match (dx, &self.color) {
                (0, Color::White) => self.position.y.checked_add(1) == Some(new_position.y),
                (0, Color::Black) => self.position.y.checked_sub(1) == Some(new_position.y),
                _ => false,
            },
            PieceType::Knight => (dx == 2 && dy == 1) || (dx == 1 && dy == 2),
            PieceType::Bishop => dx == dy,
            PieceType::Rook => dx == 0 || dy == 0,
            PieceType::Queen => (dx == 0 || dy == 0) || (dx == dy),
            PieceType::King => dx <= 1 && dy <= 1,
        };

        if is_valid {
            self.position = new_position;
        }

        is_valid
    }
}

fn main() {
    println!("Hello, world!");
}
