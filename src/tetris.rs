pub use self::tetris::Game;

mod tetris {
    use rand::{thread_rng, Rng};
    use std::cmp::min;
    
    // Private Data
    
    // Global constant for the number of cordinates in a Tetrimino
    const CORD_NUM: usize = 3; // Doesn't count the center
    const NUM_MINOS: usize = 3; // Number of held minos
    const B_LEN: usize = 10; // Board Sizes (Length and Height)
    const B_HEI: usize = 22;

    const CORD_LIST: [[[i8; 2]; 3]; 7] =  
    // Shape O
    [
        [
            [1, 0],
            [0, 1],
            [1, 1]
        ], 
        
        // Shape I
        [
            [-1, 0],
            [1, 0],
            [2, 0]
        ],
        
        // Shape S
        [
            [-1, 0],
            [0, 1],
            [1, 1]
        ],
        
        // Shape Z
        [
            [1, 0],
            [0, 1],
            [-1, 1]
        ], 
        
        // Shape L
        [
            [-1, 0],
            [1, 0],
            [1, 1]
        ],
        
        // Shape J
        [
            [1, 0],
            [-1, 0],
            [-1, 1]
        ],
        
        // Shape T
        [
            [-1, 0],
            [0, 1],
            [1, 0]
        ]
    ];

    const BLACK: usize = 7;

    const COLOR_LIST: [[u8; 3]; 8] = [
        [236, 217, 50], // Shape O = Yellow
        [235, 128, 32], // Shape I = Orange
        [82, 224, 82],  // Shape S = Green
        [229, 68, 46],  // Shape Z = Red
        [42, 255, 223], // Shape L = Cyan
        [34, 142, 255], // Shape J = Blue
        [186, 35, 230], // Shape T = Purple
        [0, 0, 0],   // Black
    ];

    const MAX_LEVEL : usize = 19;
    const DROP_TICKS : [u16; MAX_LEVEL+1] = [  
      // 0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
        30, 28, 26, 24, 22, 20, 18, 16, 14, 12,
      //10, 11, 12, 13, 14, 15, 16, 17, 18, 19
        10,  9,  8,  7,  6,  5,  4,  3,  2,  1
    ];


    // Possible shapes
    #[derive(Copy, Clone)]
    enum Shape {
        ShapeO,
        ShapeI,
        ShapeS,
        ShapeZ,
        ShapeL,
        ShapeJ,
        ShapeT,
    }

    impl Default for Shape {
        fn default() -> Self {
            Shape::ShapeO
        }
    }

    // Possible orientations Up, Left, Right and Down
    #[derive(Copy, Clone)]
    enum Orientation {
        OriU,
        OriR,
        OriD,
        OriL,
    }

    impl Default for Orientation {
        fn default() -> Self {
            Orientation::OriU
        }
    }

    // Struct of a Tetrimino
    #[derive(Default, Copy, Clone)]
    struct Mino {
        // Ints to hold the values of the shape and orientation
        // Values are only to be thoses in the corresponding enum
        shape: Shape,
        ori: Orientation,

        // The center cordinate of the Tetrimino
        // Is used as the origin of rotation
        // I shape swaps its cordinate depending on the orientation
        center: [u8; 2],

        // 2D array that holds the cordinates of spaces the Tetrimino is in
        // except for the center cord
        cords: [[u8; 2]; 3],
    }

    impl Mino {
        // Creator of my structures
        fn new() -> Mino {
            let mut piece: Mino = Default::default();

            let cent_x: i8 = 5;
            let cent_y: i8 = 20;

            let mut rng = thread_rng();
            let mut rand: u32 = rng.gen();
            rand = rand % 7;
            piece.shape = match rand {
                1 => Shape::ShapeI,
                2 => Shape::ShapeS,
                3 => Shape::ShapeZ,
                4 => Shape::ShapeL,
                5 => Shape::ShapeJ,
                6 => Shape::ShapeT,
                _ => Shape::ShapeO,
            };

            piece.ori = Orientation::OriD;

            piece.center[0] = cent_x as u8;
            piece.center[1] = cent_y as u8;

            for i in 0..CORD_NUM {
                piece.cords[i][0] = (cent_x + CORD_LIST[piece.shape as usize][i][0]) as u8;

                piece.cords[i][1] = (cent_y + CORD_LIST[piece.shape as usize][i][1]) as u8;
            }

            return piece;
        }

        // Turns the piece 90 degree clockwise around its center index
        // I shape changes the center index and O shape can't rotate
        fn rotate(&mut self) {
            let cent_x = self.center[0] as i16;
            let cent_y = self.center[1] as i16;

            for i in 0..CORD_NUM {
                let curr_x = self.cords[i][0] as i16;
                let curr_y = self.cords[i][1] as i16;

                self.cords[i][0] = (cent_x + (curr_y - cent_y)) as u8;
                self.cords[i][1] = (cent_y - (curr_x - cent_x)) as u8;
            }

            self.ori = match self.ori {
                Orientation::OriU => Orientation::OriR,
                Orientation::OriR => Orientation::OriD,
                Orientation::OriD => Orientation::OriL,
                Orientation::OriL => Orientation::OriU,
            };

            match self.shape {
                Shape::ShapeI => {
                    if self.ori as i32 % 2 == 0 {
                        let temp_y = self.center[1];
                        self.center[1] = self.cords[1][1];
                        self.cords[1][1] = temp_y;
                    }
                }
                _ => {}
            }
        }

        // Moves the piece left or right one with false being left and true being right
        fn shift(&mut self, dir: bool) {
            if dir {
                self.center[0] += 1;
            } else {
                self.center[0] -= 1;
            }

            for i in 0..CORD_NUM {
                if dir {
                    self.cords[i][0] += 1;
                } else {
                    self.cords[i][0] -= 1;
                }
            }
        }

        // Moves the piece down one
        fn fall(&mut self) {
            self.center[1] -= 1;

            for i in 0..CORD_NUM {
                self.cords[i][1] -= 1;
            }
        }
    }

    // Possible States of Block in Game Board
    #[derive(PartialEq, Copy, Clone)]
    enum State {
        Empty,
        Falling,
        Placed,
        Phantom,
    }

    impl Default for State {
        fn default() -> Self {
            State::Empty
        }
    }

    // Public Data
    #[derive(Default, Copy, Clone)]
    pub struct Block {
        state: State,
        pub color: [u8; 3],
    }

    // Struct of the Game
    // Holds the info of everything used in the game of Tetris
    #[derive(Default)]
    pub struct Game {
        // 2D array that holds whether a that spaces is taken or not on the game board
        pub board: [[Block; 22]; 10],
        pub next_board: [[[u8; 3]; 10]; 4],
        pub held_board: [[[u8; 3]; 4]; 4],

        // Tetriminos that are to be displayed in the game
        curr_mino: Mino,
        next_minos: [Mino; 3], // The next the 3 Tetriminos that will be dropped
        held_mino: Mino,

        // Boolean to know if there is a held mino
        has_held: bool,

        // Invisble componenets of Tetris
        level: usize,
        pub drop_ticks: u16, // Called speed but actually Milliseconds

        // Displayed values for the game
        pub lines: u16,
        pub time: u16, // Total Seconds
        pub score: u32,
    }

    impl Game {
        // Constructor
        pub fn new() -> Game {
            let mut tetris = Game {
                curr_mino : Mino::new(),
                next_minos : [Mino::new(), Mino::new(), Mino::new()],
                drop_ticks : DROP_TICKS[0],
                .. Default::default()
            };
            tetris.update_pos();
            tetris.update_preview();
            tetris
        }

        fn can_rotate(&self) -> bool {
            let piece = &self.curr_mino;

            match piece.shape {
                Shape::ShapeO => {
                    return false;
                }
                _ => {}
            }

            let cent_x = piece.center[0] as i16;
            let cent_y = piece.center[1] as i16;

            for i in 0..CORD_NUM {
                let curr_x = piece.cords[i][0] as i16;
                let curr_y = piece.cords[i][1] as i16;

                let new_x = cent_x + (curr_y - cent_y);
                let new_y = cent_y - (curr_x - cent_x);

                if new_x >= B_LEN as i16
                    || new_x < 0
                    || new_y < 0
                    || self.board[new_x as usize][new_y as usize].state == State::Placed
                {
                    return false;
                }
            }
            return true;
        }

        /*
         * This function is for the user, aka main, to call.
         * Technically, this could be done within the function
         * currently called "can_rotate". However, I find this way
         * to have a better naming convention that would be understandable
         * for myself and hopefully others, if they read this.
         */
        pub fn rotate(&mut self) {
            if self.can_rotate() {
                self.curr_mino.rotate();
                self.update_pos();
            }
        }

        fn can_shift(&self, dir: bool) -> bool {
            let mov: i8 = if dir { 1 } else { -1 };

            let piece = &self.curr_mino;

            let mut x = piece.center[0] as i8;
            let mut y = piece.center[1] as usize;

            let mut next_x = x + mov;

            if next_x >= B_LEN as i8
                || next_x < 0
                || self.board[next_x as usize][y].state == State::Placed
            {
                return false;
            }

            for i in 0..CORD_NUM {
                x = piece.cords[i][0] as i8;
                y = piece.cords[i][1] as usize;

                next_x = x + mov;

                if next_x >= B_LEN as i8
                    || next_x < 0
                    || self.board[next_x as usize][y].state == State::Placed
                {
                    return false;
                }
            }
            return true;
        }

        pub fn shift(&mut self, dir: bool) {
            if self.can_shift(dir) {
                self.curr_mino.shift(dir);
                self.update_pos();
            }
        }

        fn place(&mut self) {
            let mut x = self.curr_mino.center[0] as usize;
            let mut y = self.curr_mino.center[1] as usize;

            self.board[x][y].state = State::Placed;

            for i in 0..CORD_NUM {
                x = self.curr_mino.cords[i][0] as usize;
                y = self.curr_mino.cords[i][1] as usize;

                self.board[x][y].state = State::Placed;
            }
        }

        // Gets the distance the current Tetrimino will drop if dropped
        fn drop_distance(&self) -> usize {
            let calc_drop = |y: usize, board: &[Block]| -> usize {
                for j in (0..y).rev() {
                    if board[j].state == State::Placed {
                        return y - (j + 1);
                    }
                }
                return y;
            };

            let piece = &self.curr_mino;

            let cx = piece.center[0] as usize;
            let cy = piece.center[1] as usize;

            let mut drop = calc_drop(cy, &self.board[cx][0..cy]);

            for i in 0..CORD_NUM {
                let x = piece.cords[i][0] as usize;
                let y = piece.cords[i][1] as usize;

                if x != cx || y < cy {
                    let temp = calc_drop(y, &self.board[x][0..y]);
                    if temp < drop {
                        drop = temp
                    }
                }
            }
            return drop;
        }

        // The updates the location of the current tetrimino
        fn update_pos(&mut self) {
            let drop = self.drop_distance();
            let piece = &self.curr_mino;

            for i in 0..B_LEN {
                for j in 0..B_HEI {
                    match self.board[i][j].state {
                        State::Falling | State::Phantom => {
                            self.board[i][j].state = State::Empty;
                            self.board[i][j].color = COLOR_LIST[BLACK];
                        }
                        _ => {}
                    }
                }
            }

            let mut x = piece.center[0] as usize;
            let mut y = piece.center[1] as usize;

            self.board[x][y - drop].state = State::Phantom;
            self.board[x][y].state = State::Falling;
            self.board[x][y].color = COLOR_LIST[piece.shape as usize];
            self.board[x][y - drop].color = COLOR_LIST[piece.shape as usize];

            for i in 0..CORD_NUM {
                x = piece.cords[i][0] as usize;
                y = piece.cords[i][1] as usize;

                self.board[x][y - drop].state = State::Phantom;
                self.board[x][y].state = State::Falling;
                self.board[x][y].color = COLOR_LIST[piece.shape as usize];
                self.board[x][y - drop].color = COLOR_LIST[piece.shape as usize];
            }
        }

        // Updates held and next minos block positions
        fn update_preview(&mut self) {
            self.next_board = [[COLOR_LIST[BLACK]; 10]; 4];
            self.held_board = [[COLOR_LIST[BLACK]; 4]; 4];

            if self.has_held {
                let cent_x = 1;
                let cent_y = 1;

                self.held_board[cent_x][cent_y] = COLOR_LIST[self.held_mino.shape as usize];

                for i in 0..CORD_NUM {
                    let x =
                        (cent_x as i8 + CORD_LIST[self.held_mino.shape as usize][i][0]) as usize;
                    let y =
                        (cent_y as i8 + CORD_LIST[self.held_mino.shape as usize][i][1]) as usize;

                    self.held_board[x][y] = COLOR_LIST[self.held_mino.shape as usize];
                }
            }

            let cent_x = 1;
            let mut cent_y: isize = 7;
            for j in 0..NUM_MINOS {
                self.next_board[cent_x][cent_y as usize] =
                    COLOR_LIST[self.next_minos[j].shape as usize];

                for i in 0..CORD_NUM {
                    let x = (cent_x as i8 + CORD_LIST[self.next_minos[j].shape as usize][i][0])
                        as usize;
                    let y = (cent_y as i8 + CORD_LIST[self.next_minos[j].shape as usize][i][1])
                        as usize;

                    self.next_board[x][y] = COLOR_LIST[self.next_minos[j].shape as usize];
                }

                cent_y -= 3;
            }
        }

        // Releases the next mino making it the current Tetrimino, in the invisible
        // rows of the board (above row 20)
        fn release_next(&mut self) {
            self.curr_mino = self.next_minos[0];
            self.next_minos[0] = self.next_minos[1];
            self.next_minos[1] = self.next_minos[2];
            self.next_minos[2] = Mino::new();
            self.update_preview();
        }

        fn all_down_one(&mut self, bottom: usize) {
            for i in 0..B_LEN {
                for j in bottom..(B_HEI - 1) {
                    self.board[i][j] = self.board[i][j + 1];
                }
            }
        }

        // Reads the possible rows to clear and if they are full clears
        fn line_up(&mut self) {
            let mut num_rows = 0;
            let mut is_filled: bool;

            let mut j: usize = 0;
            while j < B_HEI {
                is_filled = true;
                for i in 0..B_LEN {
                    if self.board[i][j].state != State::Placed {
                        is_filled = false;
                        break;
                    }
                }
                j += 1;

                if is_filled {
                    j -= 1;
                    self.all_down_one(j);
                    self.lines += 1;
                    num_rows += 1;
                }

                if num_rows >= 4 {
                    break;
                }
            }

            if self.lines / (4 * (self.level + 1)) as u16 != 0 {
                self.level = min(MAX_LEVEL, self.level +1);
            }
        }

        fn can_fall_one(&self) -> bool {
            let piece = &self.curr_mino;

            let cx = piece.center[0] as usize;
            let cy = piece.center[1] as usize;

            if cy <= 0 || self.board[cx][cy - 1].state == State::Placed {
                return false;
            }

            for i in 0..CORD_NUM {
                let x = piece.cords[i][0] as usize;
                let y = piece.cords[i][1] as usize;

                if x == cx && y > cy {
                    continue;
                } else if y <= 0 || self.board[x][y - 1].state == State::Placed {
                    return false;
                }
            }
            return true;
        }

        pub fn fall_or_place(&mut self) {
            self.drop_ticks -= 1;
            if self.drop_ticks <= 0 {
                if self.can_fall_one() {
                    self.curr_mino.fall();
                } else {
                    self.place();
                    self.line_up();
                    self.release_next();
                }
                self.drop_ticks = DROP_TICKS[self.level];
                self.update_pos();
            }
        }

        // Drops the current Tetrimino on top of the pieces below it
        pub fn drop(&mut self) {
            let dist = self.drop_distance() as u8;
            let piece = &mut self.curr_mino;

            piece.center[1] -= dist;

            for i in 0..CORD_NUM {
                piece.cords[i][1] -= dist;
            }

            self.update_pos();
        }

        // Stores the current Tetrimino and releases the previously held Tetrimino
        // if there is one. Otherwise releases the next Tetrimino
        pub fn hold(&mut self) {
            let cent_x: i8 = 5;
            let cent_y: i8 = 20;

            if !self.has_held {
                self.held_mino = self.curr_mino;
                self.release_next();
                self.has_held = true;
            } else {
                let temp = self.curr_mino;
                self.curr_mino = self.held_mino;
                self.held_mino = temp;
            }

            let piece = &mut self.held_mino;

            piece.center[0] = cent_x as u8;
            piece.center[1] = cent_y as u8;

            for i in 0..CORD_NUM {
                piece.cords[i][0] = (cent_x + CORD_LIST[piece.shape as usize][i][0]) as u8;

                piece.cords[i][1] = (cent_y + CORD_LIST[piece.shape as usize][i][1]) as u8;
            }

            self.update_pos();
            self.update_preview();
        }
    }
}
