use crate::utils::{
    data::{Board, Piece, Rotation, PieceLocation},
    game::{CheeseGame}
};

macro_rules! ploc {
    ($heights: ident, $piece: ident, $(($rot: ident, [$($x: literal),*])),*) => {
        vec![
            $(
                $(
                    {
                        let mut loc = PieceLocation {
                            rotation: Rotation::$rot,
                            x: $x,
                            y: 21,
                            $piece
                        };
                        loc.drop(&$heights);
                        loc
                    }
                ),*
            ),*
        ]
    };
}

pub fn movegen_piece(board: &Board, piece: Piece) -> Vec<PieceLocation> {
    let heights = board.0.cols.map(|x| 64 - x.leading_zeros() as i8);
    match piece {
        Piece::O => ploc!(heights, piece, (North, [0,1,2,3,4,5,6,7,8])),
        Piece::I => ploc!(heights, piece, (North, [1,2,3,4,5,6,7]), (East, [0,1,2,3,4,5,6,7,8,9])),
        Piece::S | Piece::Z => ploc!(heights, piece, (North, [1,2,3,4,5,6,7,8]), (East, [0,1,2,3,4,5,6,7,8])),
        Piece::T | Piece::J | Piece::L => ploc!(heights, piece, (North, [1,2,3,4,5,6,7,8]), (East, [0,1,2,3,4,5,6,7,8]), (South, [1,2,3,4,5,6,7,8]), (West, [1,2,3,4,5,6,7,8,9])),
    }
}

// type Path = (Vec<PieceLocation>, CheeseGame);
// pub fn search(game: &CheeseGame, queue: &Vec<Piece>) -> Vec<PieceLocation> {
//     let mut current: Vec<Path> = vec![(vec![], game.clone())];
//     for p in 0..13 {
//         let mut new_vec: Vec<Path> = Vec::with_capacity(current.len() * 10);
//         for cheese_game in current.iter() {
//             let last_game = cheese_game.clone().1;
//             let yes = last_game.game.hold.is_some() as usize;
//             for loc in movegen_piece(&last_game.game.board, queue[p + yes]).iter().chain(movegen_piece(&last_game.game.board, last_game.game.hold.unwrap_or_else(|| queue[p + 1])).iter()) {
//                 let cgarb =  loc.blocks().iter().any(|&(_, y)| y < last_game.height);
//                 let new_height = last_game.height - cgarb as i8;
//                 let new_wasted = last_game.wasted + !cgarb as u8;
//                 if new_height == 0 {
//                     return cheese_game.clone().0
//                 }
//                 if new_wasted > 3 {
//                     continue;
//                 }
//                 let mut c = last_game.clone();
//                 if (c.game.hold.is_none() && loc.piece == queue[p + 1]) || (c.game.hold.is_some() && loc.piece == c.game.hold.unwrap()) {
//                     c.game.hold = Some(queue[p + yes]);
//                 }
//                 c.height = new_height;
//                 c.wasted = new_wasted;
//                 c.game.advance(queue[p + yes], loc);
//                 let mut newv = cheese_game.clone();
//                 newv.0.push(loc.clone());
//                 newv.1 = c;
//                 new_vec.push(newv);
//             }
//         }
//         current.clear();
//         std::mem::swap(&mut new_vec, &mut current);
//     }
//     vec![]
// }

pub fn search(game: &CheeseGame, n: usize, queue: &Vec<Piece>, garb_cols: &[i8; 10]) -> Option<Vec<PieceLocation>> {
    if n > 12 { return None; }

    if game.game.board.max_height() > game.height + 4 {
        return None;
    }

    let locs = movegen_piece(&game.game.board, queue[n]).into_iter().chain(movegen_piece(&game.game.board, game.game.hold.unwrap_or_else(|| queue[n + 1])).into_iter());
    let mut games: Vec<(i8, PieceLocation, CheeseGame)> = Vec::with_capacity(64);
    'loc_loop: for loc in locs {
        let cgarb =  loc.check_garb(game.height);
        let new_height = game.height - cgarb as i8;
        let new_wasted = game.wasted + !cgarb as u8;
        if new_height == 0 {
            return Some(vec![loc.clone()]);
        }
        if new_wasted > 2 {
            continue;
        }

        let mut new_game = game.clone();
        new_game.advance(queue[n], queue[n + 1], &loc);
        for q in (0..new_height).rev() {
            let garb_col = garb_cols[q as usize];
            let max_pieces_allowed = 13 - q;
            if new_game.game.board.volume_math(garb_col, q) > max_pieces_allowed {
                continue 'loc_loop;
            }
        }
        games.push((new_game.game.board.volume_math(garb_cols[0], 0), loc, new_game));
    }

    games.sort_by_key(|(s, _, g)| -(g.wasted as i8) - *s);
    games.reverse();

    for (_, loc, new_game) in games.into_iter() {
        if let Some(mut v) = search(&new_game, n + 1 + (loc.piece == queue[n + 1]) as usize, queue, garb_cols) {
            v.push(loc);
            return Some(v);
        }
    }
    None
}