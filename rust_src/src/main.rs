use cheese13::search::search;
use cheese13::utils::{
    data::{Board, Piece, PieceLocation},
    game::{CheeseGame, Game},
};

use rand::Rng;
use rand::prelude::SliceRandom;

use serde::{Serialize, Deserialize};

fn gen_queue(bags: u32) -> Vec<Piece> {
    let mut rng = rand::rng();
    let bag = [
        Piece::I,
        Piece::J,
        Piece::L,
        Piece::O,
        Piece::S,
        Piece::T,
        Piece::Z,
    ];
    let mut queue: Vec<Piece> = vec![];
    for _ in 0..bags {
        let mut new_bag = bag.to_vec();
        new_bag.shuffle(&mut rng);
        queue.extend(new_bag);
    }
    queue
}

#[derive(Serialize, Deserialize)]
struct OutObj {
    tries: usize,
    garb_cols: [i8; 10],
    queue: Vec<Piece>,
    locs: Vec<PieceLocation>
}

fn main() {
    let mut rng = rand::rng();
    loop {
        let mut cnt = 0;
        loop {
            // println!("checking...");
            cnt += 1;
            let mut game = CheeseGame {
                wasted: 0,
                height: 10,
                game: Game {
                    board: Board::new([0u64; 10]),
                    hold: None
                }
            };

            let mut garbs = [0i8; 10];
            let mut last_garb_idx: i8 = 0;
            for i in 0..10 {
                let idx = rng.random_range(0..9);
                let idx = if idx >= last_garb_idx { idx + 1 } else { idx };
                game.game.board.push_garbage(idx);
                garbs[i] = idx;
                last_garb_idx = idx;
            }
            let queue = gen_queue(2);

            let success = search(&game, 0, &queue, &garbs);
            if let Some(v) = success {
                let mut newv = v.clone();
                newv.reverse();
                println!("{}", serde_json::to_string(&OutObj {
                    tries: cnt,
                    garb_cols: garbs,
                    queue,
                    locs: newv
                }).unwrap());
                // for l in newv {
                //     let mut outstr: Vec<String> = vec![];
                //     for y in (0..20).rev() {
                //         let mut vstr = String::new();
                //         for x in 0..10 {
                //             vstr.push_str(
                //                 if (game.game.board.0.cols[x as usize] & (1 << y)) > 0 {
                //                     "🟩"
                //                 }
                //                 else if l.blocks().iter().any(|(bx, by)| *bx == x && *by == y) { "🟥" }
                //                 else { "⬜️" }
                //             );
                //         }
                //         outstr.push(vstr);
                //     }
                //     game.advance(Piece::Z, Piece::Z, &l);
                //     println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n{}", outstr.join("\n"));
                //     std::thread::sleep(std::time::Duration::from_millis(100));
                // }
                break;
            }
        }
    }
}

// fn main() {
//     let mut rng = rand::rng();
//     let mut game = CheeseGame {
//         wasted: 0,
//         height: 10,
//         game: Game {
//             board: Board::new([0u64; 10]),
//             hold: None,
//         },
//     };

//     let mut garbs = [0usize; 10];
//     let mut last_garb_idx: usize = 0;
//     for i in 0..10 {
//         let idx = rng.random_range(0..9);
//         let idx = if idx >= last_garb_idx { idx + 1 } else { idx };
//         game.game.board.push_garbage(idx);
//         garbs[i] = idx;
//         last_garb_idx = idx;
//     }
//     let queue = gen_queue(2);

//     let success = search(&game, None, 0, &queue);
//     println!("{:?}", success);
// }
