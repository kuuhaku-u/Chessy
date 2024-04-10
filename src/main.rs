
static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
static FRN_STRING: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

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
    position:Vec<u32>,
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
    s.chars().map(|c| c.to_string()).collect()
}

fn set_board(board: &[String]) {
    for part in FRN_STRING.split('/') {
        if part.chars().all(|c| c.is_digit(10)) {
            println!();
            for _ in 0..8 {
                print!("  _  | ");
            }
            println!();
        } else {
            println!();

            let a = split_word(part);
            println!("| {} |", a.join("  |   "));
        }
    }
}


fn get_current_square(pos:&str){




}

fn eval(crr: &str, next: &str) -> bool {
    println!("{},{}", crr, next);
 
    get_current_square(&crr);

 
    true
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
    let mut cur_position = &my_move.current_position;
    let mut new_position = &my_move.new_position;
    
    eval(&cur_position, &new_position);

    // if board.contains(new_position) {
    //     println!("VALID {:#?}", my_move);
    // } else {
    //     println!("NOT VALID {:#?}", my_move);
    // }
}
