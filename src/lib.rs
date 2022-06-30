pub mod action;
pub mod state;

#[cfg(test)]
mod tests {
    use crate::state;

    #[test]
    fn it_works() {
        let mut state = state::State::new(None, None, 0);
        assert_eq!(state.is_done(), false);
        assert_eq!(state.is_draw(), false);
        assert_eq!(state.is_first_player(), true);
        assert_eq!(state.is_lose(), false);
        assert_eq!(state.piece_count(&state.pieces), 2);
        assert_eq!(state.piece_count(&state.enemy_pieces), 2);

        let mut pieces = [0; 36];
        pieces[14] = 1;
        pieces[21] = 1;
        assert_eq!(&state.pieces, &pieces);

        let mut enemy_pieces = [0; 36];
        enemy_pieces[15] = 1;
        enemy_pieces[20] = 1;
        assert_eq!(&state.enemy_pieces, &enemy_pieces);

        assert_eq!(state.depth, 0);

        state = state.next(16);

        let mut pieces = [0; 36];
        pieces[20] = 1;
        assert_eq!(&state.pieces, &pieces);

        let mut enemy_pieces = [0; 36];
        enemy_pieces[14] = 1;
        enemy_pieces[15] = 1;
        enemy_pieces[16] = 1;
        enemy_pieces[21] = 1;
        assert_eq!(&state.enemy_pieces, &enemy_pieces);
    }
}
