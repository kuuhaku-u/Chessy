static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
// const  FRN_STRING:String = String::from("ffff");
static FRN_STRING: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";



/**
 * ENUMS
 */
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

// enum PieceOrder {

    
// }

/**
 * STRUCTS
 */
#[derive(Debug)]

struct Move {
    CurrentPosition: String,
    NewPosition: String,
}

struct GameInfo {
    piece: Vec<Piece>,
    square: Vec<GameSquare>,
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

/**
 * UTILITY
 */

fn index_to_position(index: usize) -> Result<String, String> {
    let column = index % 8;
    let row = index / 8 + 1;
    // if column < 8 && row < 9 {
    //     Ok(format!("{}{}", COL_MAP[column], row))
    // } else {
    //     Err("OUT OF BOUNDS".to_string())
    // }
    match (column, row) {
        (0..=7, 1..=8) => Ok(format!("{}{}", COL_MAP[column], row)),
        _ => Err("OUT OF BOUNDS".to_string()),
    }
}

fn set_board(board: &Vec<String>) -> Vec<u32> {




    let a = [1];
    a.to_vec()
}

/**
 * MAIN
 */
fn main() {
    let mut board: Vec<String> = Vec::new();

    for i in 0..64 {
        let position_result = index_to_position(i);
        match position_result {
            Ok(position) => board.push(position),
            Err(err) => println!("{} -> Error: {}", i, err),
        }
    }
    set_board(&board);
    let my_move: Move = Move {
        CurrentPosition: "a1".to_string(),
        NewPosition: "a4".to_string(),
    };

    let a = &my_move.NewPosition;
    println!("{}", a);

    // if a.iter().any(|&x| x == 3) {
    if board.contains(a) {
        println!("VALID {:#?}", my_move)
    } else {
        println!("NOT VALID {:#?}", my_move)
    }

    // let position_result = index_to_position(12);
    // let game_piece1 = match position_result {
    //     Ok(position) => Piece {
    //         color: Color::White,
    //         class: PieceClass::Pawn,
    //         position: position,
    //     },
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         return;
    //     }
    // };
}
