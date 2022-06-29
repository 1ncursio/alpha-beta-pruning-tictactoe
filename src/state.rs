pub struct State {
    pieces: [u8; 9],
    enemy_pieces: [u8; 9],
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
    pub fn new(pieces: Option<[u8; 9]>, enemy_pieces: Option<[u8; 9]>) -> State {
        let pieces = match pieces {
            Some(pieces) => pieces,
            None => [0; 9],
        };
        let enemy_pieces = match enemy_pieces {
            Some(enemy_pieces) => enemy_pieces,
            None => [0; 9],
        };

        State {
            pieces,
            enemy_pieces,
        }
    }

    pub fn piece_count(&self, pieces: &[u8; 9]) -> usize {
        // count the number of pieces where element == 1
        pieces.iter().filter(|&x| *x == 1).count()
    }

    pub fn is_lose(&self) -> bool {
        if self.is_comp(0, 0, 1, 1) || self.is_comp(0, 2, 1, -1) {
            return true;
        }

        for i in 0..3 {
            if self.is_comp(0, i, 1, 0) || self.is_comp(i, 0, 0, 1) {
                return true;
            }
        }

        false
    }

    pub fn is_comp(&self, mut x: i8, mut y: i8, dx: i8, dy: i8) -> bool {
        for _ in 0..3 {
            if y < 0 || y > 2 || x < 0 || x > 2 || self.enemy_pieces[(x + y * 3) as usize] == 0 {
                return false;
            }
            x += dx;
            y += dy;
        }

        true
    }

    pub fn is_draw(&self) -> bool {
        self.piece_count(&self.pieces) + self.piece_count(&self.enemy_pieces) == 9
    }

    pub fn is_done(&self) -> bool {
        self.is_lose() || self.is_draw()
    }

    pub fn next(&self, action: u8) -> State {
        let mut pieces = self.pieces.clone();
        pieces[action as usize] = 1;
        State::new(Some(self.enemy_pieces.clone()), Some(pieces))
    }

    pub fn legal_actions(&self) -> Vec<u8> {
        let mut actions: Vec<u8> = vec![];

        for i in 0..9 {
            if self.pieces[i] == 0 && self.enemy_pieces[i] == 0 {
                actions.push(i as u8);
            }
        }

        actions
    }

    pub fn is_first_player(&self) -> bool {
        self.piece_count(&self.pieces) == self.piece_count(&self.enemy_pieces)
    }
}
