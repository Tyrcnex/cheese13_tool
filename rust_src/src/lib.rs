pub mod utils {
    pub mod board;
    pub mod data;
    pub mod game;
}

pub mod search;

use crate::search::search;
use crate::utils::{
    data::{Board, Piece, PieceLocation},
    game::{CheeseGame, Game},
};

use rand::Rng;
use rand::prelude::SliceRandom;

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

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
pub struct CheeseMap {
    pub tries: usize,
    pub garb_cols: Vec<i8>,
    pub queue: Vec<Piece>,
    pub locs: Vec<PieceLocation>
}

#[wasm_bindgen]
pub fn mapgen() -> JsValue {
    let mut rng = rand::rng();
    let mut cnt = 0;
    loop {
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
            return serde_wasm_bindgen::to_value(&CheeseMap {
                tries: cnt,
                garb_cols: garbs.to_vec(),
                queue,
                locs: newv
            }).unwrap();
        }
    }
}
