pub const DXY: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub struct State {
    pub pieces: [u8; 36],
    pub enemy_pieces: [u8; 36],
    pub depth: i32,
    pub pass_end: bool,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ox = if self.is_first_player() {
            ("o", "x")
        } else {
            ("x", "o")
        };

        let mut board = String::new();
        for i in 0..9 {
            if self.pieces[i] == 1 {
                board.push_str(&ox.0);
            } else if self.enemy_pieces[i] == 1 {
                board.push_str(&ox.1);
            } else {
                board.push_str("-");
            }
            if i % 3 == 2 {
                board.push_str("\n");
            }
        }
        write!(f, "{}", board)
    }
}

impl State {
    pub fn new(pieces: Option<[u8; 36]>, enemy_pieces: Option<[u8; 36]>, depth: i32) -> State {
        let pieces = match pieces {
            Some(pieces) => pieces,
            None => {
                let mut pieces = [0; 36];
                pieces[14] = 1;
                pieces[21] = 1;
                pieces
            }
        };
        let enemy_pieces = match enemy_pieces {
            Some(enemy_pieces) => enemy_pieces,
            None => {
                let mut enemy_pieces = [0; 36];
                enemy_pieces[15] = 1;
                enemy_pieces[20] = 1;
                enemy_pieces
            }
        };

        State {
            pieces,
            enemy_pieces,
            depth,
            pass_end: false,
        }
    }

    pub fn piece_count(&self, pieces: &[u8; 36]) -> usize {
        // count the number of pieces where element == 1
        pieces.iter().filter(|&x| *x == 1).count()
    }

    pub fn is_lose(&self) -> bool {
        self.is_done() && self.piece_count(&self.pieces) < self.piece_count(&self.enemy_pieces)
    }

    pub fn is_draw(&self) -> bool {
        self.is_done() && self.piece_count(&self.pieces) == self.piece_count(&self.enemy_pieces)
    }

    pub fn is_done(&self) -> bool {
        self.pass_end || self.piece_count(&self.pieces) + self.piece_count(&self.enemy_pieces) == 36
    }

    pub fn next(&self, action: u8) -> State {
        // println!("next!");
        let mut state = State::new(
            Some(self.pieces.clone()),
            Some(self.enemy_pieces.clone()),
            self.depth + 1,
        );
        if action != 36 {
            state.is_legal_action_xy((action % 6) as i8, (action / 6) as i8, true);
        }

        let w = state.pieces;
        state.pieces = state.enemy_pieces;
        state.enemy_pieces = w;

        if action == 36 && state.legal_actions()[0] == 36 {
            state.pass_end = true;
        }
        state
    }

    pub fn legal_actions(&mut self) -> Vec<u8> {
        let mut actions: Vec<u8> = vec![];

        for j in 0..6 {
            for i in 0..6 {
                if self.is_legal_action_xy(i, j, false) {
                    actions.push((i + j * 6) as u8);
                }
            }
        }

        if actions.len() == 3 {
            actions.push(36);
        }

        actions
    }

    fn is_legal_action_xy(&mut self, x: i8, y: i8, flip: bool) -> bool {
        if self.enemy_pieces[(x + y * 6) as usize] == 1 || self.pieces[(x + y * 6) as usize] == 1 {
            return false;
        }

        if flip {
            self.pieces[(x + y * 6) as usize] = 1;
        }

        let mut flag = false;
        for (dx, dy) in DXY {
            if self.is_legal_action_xy_dxy(x, y, dx, dy, flip) {
                flag = true;
            }
        }

        flag
    }

    fn is_legal_action_xy_dxy(&mut self, mut x: i8, mut y: i8, dx: i8, dy: i8, flip: bool) -> bool {
        x += dx;
        y += dy;
        if y < 0 || 5 < y || x < 0 || 5 < x || self.enemy_pieces[(x + y * 6) as usize] != 1 {
            return false;
        }

        for _ in 0..6 {
            if y < 0
                || 5 < y
                || x < 0
                || 5 < x
                || (self.enemy_pieces[(x + y * 6) as usize] == 0
                    && self.pieces[(x + y * 6) as usize] == 0)
            {
                return false;
            }

            if self.pieces[(x + y * 6) as usize] == 1 {
                if flip {
                    for _ in 0..6 {
                        x -= dx;
                        y -= dy;
                        if self.pieces[(x + y * 6) as usize] == 1 {
                            return true;
                        }
                        self.pieces[(x + y * 6) as usize] = 1;
                        self.enemy_pieces[(x + y * 6) as usize] = 0;
                    }
                }
                return true;
            }
            x += dx;
            y += dy;
        }

        false
    }

    pub fn is_first_player(&self) -> bool {
        self.depth % 2 == 0
    }
}
