use std::{fmt, io::Empty, iter::empty, ptr::null};

#[test]
fn test_print_board() 
{
    let mut game = Game::new();

    game.reset_board();

    let moves = game.get_possible_moves(1, 6);
    for m in moves
    {
        println!("{:?}", m);
    }

    game.print_board();
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState 
{
    InProgress,
    Check,
    GameOver
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType 
{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color
{
    White,
    Black
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece
{
    pieceType: PieceType,
    color: Color
}

const KnHOPS: [(isize, isize); 8] = 
[
              ( -1, -2),           ( 1, -2), 
    (-2, -1),                               ( 2, -1),

    (-2,  1),                               ( 2,  1),
              ( -1,  2),           ( 1,  2)
];
const BDIRS: [(isize, isize); 4] = 
[
    (-1, -1),           ( 1, -1),
    
    (-1,  1),           ( 1,  1)
];
const RDIRS: [(isize, isize); 4] = 
[
              ( 0, -1), 
    (-1,  0),           ( 1,  0), 
              ( 0,  1)
];
const QKDIRS: [(isize, isize); 8] = 
[
    (-1, -1), ( 0, -1), ( 1, -1),
    (-1,  0),           ( 1,  0), 
    (-1,  1), ( 0,  1), ( 1,  1)
];


pub struct Game 
{
    /* save board, active colour, ... */
    state: GameState,
    board: [[Option<Piece>; 8]; 8]
    //...
}

impl Game 
{
    /// Initialises a new board with pieces.
    pub fn new() -> Game 
    {
        Game 
        {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: [[None; 8]; 8]
            //...
        }
    }

    pub fn reset_board(&mut self)
    {
        for x in 0..8
        {
            self.board[x][1] = Some(Piece { pieceType: PieceType::Pawn, color: Color::Black });
            self.board[x][6] = Some(Piece { pieceType: PieceType::Pawn, color: Color::White });
        }

        self.board[0][0] = Some(Piece { pieceType: PieceType::Rook,   color: Color::Black });
        self.board[1][0] = Some(Piece { pieceType: PieceType::Knight, color: Color::Black });
        self.board[2][0] = Some(Piece { pieceType: PieceType::Bishop, color: Color::Black });
        self.board[3][0] = Some(Piece { pieceType: PieceType::King,   color: Color::Black });
        self.board[4][0] = Some(Piece { pieceType: PieceType::Queen,  color: Color::Black });
        self.board[5][0] = Some(Piece { pieceType: PieceType::Bishop, color: Color::Black });
        self.board[6][0] = Some(Piece { pieceType: PieceType::Knight, color: Color::Black });
        self.board[7][0] = Some(Piece { pieceType: PieceType::Rook,   color: Color::Black });

        self.board[0][7] = Some(Piece { pieceType: PieceType::Rook,   color: Color::White });
        self.board[1][7] = Some(Piece { pieceType: PieceType::Knight, color: Color::White });
        self.board[2][7] = Some(Piece { pieceType: PieceType::Bishop, color: Color::White });
        self.board[3][7] = Some(Piece { pieceType: PieceType::King,   color: Color::White });
        self.board[4][7] = Some(Piece { pieceType: PieceType::Queen,  color: Color::White });
        self.board[5][7] = Some(Piece { pieceType: PieceType::Bishop, color: Color::White });
        self.board[6][7] = Some(Piece { pieceType: PieceType::Knight, color: Color::White });
        self.board[7][7] = Some(Piece { pieceType: PieceType::Rook,   color: Color::White });

        self.board[4][4] = Some(Piece { pieceType: PieceType::Rook,   color: Color::White });
        self.board[3][3] = Some(Piece { pieceType: PieceType::Bishop, color: Color::Black });
        self.board[4][2] = Some(Piece { pieceType: PieceType::Queen, color: Color::Black });
        self.board[3][4] = Some(Piece { pieceType: PieceType::Knight, color: Color::White });
        self.board[2][4] = Some(Piece { pieceType: PieceType::Pawn, color: Color::White });
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _fromX: usize,_fromY: usize, _toX: usize, _toY: usize) -> Option<GameState> 
    {
        None
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _posX: usize, _posY: usize) -> () 
    {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState 
    {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _posX: usize, _posY: usize) -> Option<Vec<(usize, usize)>> 
    {
        let mut moves = Vec::new();

        let piece = self.board[_posX][_posY].unwrap();

        match piece.pieceType {
            PieceType::Pawn => 
            {
                let mut x = _posX as isize;
                let mut y = _posY as isize + if piece.color == Color::White {-1} else {1};

                if x >= 0 && y >= 0 && x <= 7 && y <= 7 
                {
                    if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                    else if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                }

                if (_posY == 6 && piece.color == Color::White) || (_posY == 1 && piece.color == Color::Black)
                {
                    y = _posY as isize + if piece.color == Color::White {-2} else {2};

                    if x >= 0 && y >= 0 && x <= 7 && y <= 7 
                    {
                        if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                        else if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                    }
                }

                x = _posX as isize - 1;
                y = _posY as isize + if piece.color == Color::White {-1} else {1};

                if x >= 0 && y >= 0 && x <= 7 && y <= 7 
                {
                    if self.board[x as usize][y as usize] != None 
                    {
                        if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                    }
                }

                x = _posX as isize + 1;

                if x >= 0 && y >= 0 && x <= 7 && y <= 7 
                {
                    if self.board[x as usize][y as usize] != None 
                    {
                        if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                    }
                }
            },
            PieceType::Knight =>  
            {
                for hop in KnHOPS
                {
                    let x = _posX as isize + hop.0;
                    let y = _posY as isize + hop.1;

                    if x >= 0 && y >= 0 && x <= 7 && y <= 7 
                    {
                        if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                        else if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                    }
                }
            },
            PieceType::Bishop =>  
            {
                for dir in BDIRS
                {
                    let mut x = _posX as isize + dir.0;
                    let mut y = _posY as isize + dir.1;

                    while x >= 0 && y >= 0 && x <= 7 && y <= 7 
                    {
                        if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                        else 
                        {
                            if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                            break;
                        }
                        x += dir.0;
                        y += dir.1;
                    }
                }
            },
            PieceType::Rook =>  
            {
                for dir in RDIRS
                {
                    let mut x = _posX as isize + dir.0;
                    let mut y = _posY as isize + dir.1;

                    while x >= 0 && y >= 0 && x <= 7 && y <= 7 
                    {
                        if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                        else 
                        {
                            if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                            break;
                        }
                        x += dir.0;
                        y += dir.1;
                    }
                }
            },
            PieceType::Queen =>  
            {
                for dir in QKDIRS
                {
                    let mut x = _posX as isize + dir.0;
                    let mut y = _posY as isize + dir.1;

                    while x >= 0 && y >= 0 && x <= 7 && y <= 7 
                    {
                        if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                        else 
                        {
                            if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                            break;
                        }
                        x += dir.0;
                        y += dir.1;
                    }
                }
            },
            PieceType::King =>  
            {
                for dir in QKDIRS
                {
                    let x = _posX as isize + dir.0;
                    let y = _posY as isize + dir.1;

                    if x >= 0 && y >= 0 && x <= 7 && y <= 7 
                    {
                        if self.board[x as usize][y as usize] == None { moves.push((x as usize, y as usize)); }
                        else if self.board[x as usize][y as usize].unwrap().color != piece.color { moves.push((x as usize, y as usize)); }
                    }
                }
            }
        }
        return Some(moves);
    }

    pub fn print_board(&self)
    {
        print!("\x1b[33m  0  1  2  3  4  5  6  7\n");
        for y in 0..8
        {
            print!("\x1b[33m");
            print!("{}", y);
            print!(" ");
            for x in 0..8
            {
                let piece = self.board[x][y];
                if piece != None
                {
                    if piece.unwrap().color == Color::White
                    {
                        print!("\x1b[37m");
                    }
                    else
                    {
                        print!("\x1b[30m");
                    }
                    if piece.unwrap().pieceType == PieceType::Pawn
                    {
                        print!("P  ");
                    }
                    if piece.unwrap().pieceType == PieceType::Knight
                    {
                        print!("Kn ");
                    }
                    if piece.unwrap().pieceType == PieceType::Bishop
                    {
                        print!("B  ");
                    }
                    if piece.unwrap().pieceType == PieceType::Rook
                    {
                        print!("R  ");
                    }
                    if piece.unwrap().pieceType == PieceType::Queen
                    {
                        print!("Q  ");
                    }
                    if piece.unwrap().pieceType == PieceType::King
                    {
                        print!("K  ");
                    }
                    print!("\x1b[0m");
                }
                else 
                {
                    print!("\x1b[30m-  \x1b[0m");
                }
            }

            print!("\n");
        }
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        /* build board representation string */
        
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests 
{
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() 
    {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() 
    {
        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}