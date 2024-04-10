static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
static FRN_STRING: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

// static FRN_STRING: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
enum Color {
    White,
    Black,
}

#[derive(Debug)]
enum PieceClass {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

enum GameSquare {
    Empty,
    Occupied(u64),
}

#[derive(Debug)]
struct Move {
    current_position: String,
    new_position: String,
}

struct GameInfo {
    pieces: Vec<Piece>,
}

#[derive(Debug)]
struct Piece {
    color: Color,
    class: PieceClass,
    position: String,
}

impl Piece {
    fn log_piece_info(&self) {
        println!(
            "{:#?} -> {:#?} -> {}",
            &self.color, &self.class, self.position
        )
    }
}

fn index_to_position(index: usize) -> Result<String, String> {
    let column = index % 8;
    let row = index / 8 + 1;

    match (column, row) {
        (0..=7, 1..=8) => Ok(format!("{}{}", COL_MAP[column], row)),
        _ => Err("OUT OF BOUNDS".to_string()),
    }
}

fn split_word(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for part in s.chars() {
        result.push(part.to_string());
    }
    result
}

fn set_board(board: &[String]) -> Vec<u32> {
    for part in FRN_STRING.split('/') {
        if part.chars().all(|c| c.is_digit(10)) {
            println!("DD");
        } else {
            let a = split_word(part);
            println!("{:?}", a);
        }
    }
    let a = [1];
    draw_board();
    a.to_vec()

}

fn draw_board() {
    for _ in 0..8 {
        for _ in 0..8 {
            print!(" _ ");
        }
        println!(); 
    }
}

fn main() {
    let mut board: Vec<String> = Vec::new();

    for i in 0..64 {
        if let Ok(position) = index_to_position(i) {
            board.push(position);
        } else {
            println!("{} -> Error: OUT OF BOUNDS", i);
        }
    }

    set_board(&board);
    let my_move = Move {
        current_position: "a1".to_string(),
        new_position: "a4".to_string(),
    };

    let new_position = &my_move.new_position;
    println!("{}", new_position);

    if board.contains(new_position) {
        println!("VALID {:#?}", my_move);
    } else {
        println!("NOT VALID {:#?}", my_move);
    }
}
