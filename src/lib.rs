mod utils;

use rand::Rng;
use std::time::Instant;
use wasm_bindgen::prelude::*;

const SHOW_STATS: bool = true;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tic-tac-toe!");
}

#[wasm_bindgen]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Field {
    X,
    O,
    Free,
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

struct Ai {
    difficulty: Difficulty,
    iterations: u64,
    depth: u64,
}

impl Ai {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            iterations: 0,
            depth: 0,
        }
    }

    pub fn make_move(&mut self, board: &mut Board) {
        // reset values
        let delta_time = Instant::now();
        self.iterations = 0;
        self.depth = 0;
        let mut rng = rand::thread_rng();
        let was_rand;

        match self.difficulty {
            Difficulty::Hard => {
                self.make_move_minmax(board);
                was_rand = false;
            }
            Difficulty::Medium => {
                if rng.gen_range(0..2) == 0 {
                    self.make_move_minmax(board);
                    was_rand = false;
                } else {
                    self.make_move_rand(board);
                    was_rand = true;
                }
            }
            Difficulty::Easy => {
                self.make_move_rand(board);
                was_rand = true;
            }
        }

        if SHOW_STATS {
            let duration = delta_time.elapsed();
            println!(
                "was_rand:{} delta_time: {}, iterations: {}, depth: {}",
                was_rand,
                (duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9),
                self.iterations,
                self.depth
            );
        }
    }

    fn make_move_rand(&mut self, board: &mut Board) {
        let mut rng = rand::thread_rng();
        let free_fealds = board.get_free();
        let feld_index = rng.gen_range(0..free_fealds.len());
        let (x, y) = free_fealds[feld_index];
        board.make_move(x, y, Field::O);
    }

    fn make_move_minmax(&mut self, board: &mut Board) {
        let mut best_score = -2;
        let mut next_move: Option<(u16, u16)> = None;
        for (x, y) in board.get_free() {
            board.make_move(x, y, Field::O);
            let score = self.minimax(board, 0, true);
            board.set(x, y, Field::Free);
            if score > best_score {
                best_score = score;
                next_move = Some((x, y));
            }
        }

        if let Some((x, y)) = next_move {
            board.make_move(x, y, Field::O);
        } else {
            panic!("invalid move");
        }
    }

    fn minimax(&mut self, board: &mut Board, depth: u64, is_max: bool) -> i16 {
        self.iterations += 1;
        if board.has_won() == Field::O {
            self.depth = depth;
            return 1;
        } else if board.has_won() == Field::X {
            self.depth = depth;
            return -1;
        } else if board.is_full() {
            self.depth = depth;
            return 0;
        }

        if is_max {
            let mut best_score = -2;
            for (x, y) in board.get_free() {
                board.make_move(x, y, Field::O);
                let score = self.minimax(board, depth + 1, false);
                board.set(x, y, Field::Free);
                if score > best_score {
                    best_score = score;
                }
            }
            best_score
        } else {
            let mut best_score = 2;
            for (x, y) in board.get_free() {
                board.make_move(x, y, Field::X);
                let score = self.minimax(board, depth + 1, true);
                board.set(x, y, Field::Free);
                if score < best_score {
                    best_score = score;
                }
            }
            best_score
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Debug, Clone)]
struct Board {
    fields: Vec<Field>,
}

impl Board {
    fn get_free(&self) -> Vec<(u16, u16)> {
        let mut result = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if self.get(x, y) == Field::Free {
                    result.push((x as u16, y as u16));
                }
            }
        }
        result
    }

    fn get(&self, x: u16, y: u16) -> Field {
        self.fields[(3 * y + x) as usize].clone()
    }

    fn set(&mut self, x: u16, y: u16, value: Field) {
        self.fields[(3 * y + x) as usize] = value;
    }
}

impl Board {
    fn has_won_player(&self, player: Field) -> bool {
        let mut diagonal1 = true;
        for x in 0..3 {
            let mut rows = true;
            let mut cols = true;
            for y in 0..3 {
                if self.get(x, y) != player {
                    cols = false;
                }
                if self.get(y, x) != player {
                    rows = false;
                }

                if x == y {
                    if self.get(x, y) != player {
                        diagonal1 = false;
                    }
                }
            }
            if rows | cols {
                return true;
            }
        }

        if diagonal1 {
            return true;
        }

        if self.get(0, 2) == player && self.get(1, 1) == player && self.get(2, 0) == player {
            return true;
        }

        false
    }
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        Board {
            fields: vec![Field::Free; 9],
        }
    }

    pub fn is_full(&self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if self.get(x, y) == Field::Free {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn make_move(&mut self, x: u16, y: u16, player: Field) {
        if self.get(x, y) != Field::Free {
            panic!("invalid move x{} y{}", y, x);
        }
        self.set(x, y, player);
    }

    pub fn has_won(&self) -> Field {
        if self.has_won_player(Field::X) {
            return Field::X;
        } else if self.has_won_player(Field::O) {
            return Field::O;
        } else {
            return Field::Free;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimax_x() {
        let mut ai = Ai::new(Difficulty::Hard);

        // __o
        // xox
        // _o_
        let mut board = Board {
            fields: vec![
                Field::Free,
                Field::Free,
                Field::O,
                Field::X,
                Field::O,
                Field::X,
                Field::Free,
                Field::O,
                Field::Free,
            ],
        };
        assert_eq!(ai.minimax(&mut board, 0, true), 1);

        // _oo
        // xox
        // _o_
        let board_won = Board {
            fields: vec![
                Field::Free,
                Field::O,
                Field::O,
                Field::X,
                Field::O,
                Field::X,
                Field::Free,
                Field::O,
                Field::Free,
            ],
        };
        ai.make_move(&mut board);
        assert_eq!(board, board_won);
    }

    #[test]
    fn test_minimax_o() {
        let mut ai = Ai::new(Difficulty::Hard);
        // _xo
        // o_x
        // ox_
        let mut board = Board {
            fields: vec![
                Field::Free,
                Field::X,
                Field::O,
                Field::O,
                Field::Free,
                Field::X,
                Field::O,
                Field::X,
                Field::Free,
            ],
        };
        assert_eq!(ai.minimax(&mut board, 0, false), -1);

        // _xo
        // oxx
        // ox_
        let board_won = Board {
            fields: vec![
                Field::Free,
                Field::X,
                Field::O,
                Field::O,
                Field::X,
                Field::X,
                Field::O,
                Field::X,
                Field::Free,
            ],
        };

        ai.make_move(&mut board);
        assert_eq!(board, board_won);
    }

    #[test]
    #[ignore]
    // due to longe testing time (delta_time: 2.603189, iterations: 456777, depth: 6)
    fn test_minimax_free() {
        let mut ai = Ai::new(Difficulty::Hard);
        let mut board = Board::new();
        assert_eq!(ai.minimax(&mut board, 0, false), 0);

        let board_won = Board {
            fields: vec![
                Field::O,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::Free,
            ],
        };

        ai.make_move(&mut board);
        assert_eq!(board, board_won);
    }

    #[test]
    fn test_is_full() {
        // __o
        // xox
        // _o_
        let board1 = Board {
            fields: vec![
                Field::Free,
                Field::Free,
                Field::O,
                Field::X,
                Field::O,
                Field::X,
                Field::Free,
                Field::O,
                Field::Free,
            ],
        };

        // xxx
        // xxx
        // xxx
        let board2 = Board {
            fields: vec![Field::X; 9],
        };

        assert_eq!(board1.is_full(), false);
        assert_eq!(board2.is_full(), true);
    }

    #[test]
    fn test_get_free() {
        // __o
        // xox
        // _o_
        let board = Board {
            fields: vec![
                Field::Free,
                Field::Free,
                Field::O,
                Field::X,
                Field::O,
                Field::X,
                Field::Free,
                Field::O,
                Field::Free,
            ],
        };

        assert_eq!(board.get_free(), vec![(0, 0), (0, 2), (1, 0), (2, 2)]);
    }

    #[test]
    fn test_row() {
        // ___
        // xxx
        // ___
        let board = Board {
            fields: vec![
                Field::Free,
                Field::Free,
                Field::Free,
                Field::X,
                Field::X,
                Field::X,
                Field::Free,
                Field::Free,
                Field::Free,
            ],
        };
        assert_eq!(board.has_won_player(Field::X), true);
    }

    #[test]
    fn test_col() {
        // x__
        // x__
        // x__
        let board = Board {
            fields: vec![
                Field::X,
                Field::Free,
                Field::Free,
                Field::X,
                Field::Free,
                Field::Free,
                Field::X,
                Field::Free,
                Field::Free,
            ],
        };
        assert_eq!(board.has_won_player(Field::X), true);
    }

    #[test]
    fn test_diagonal1() {
        // x__
        // _x_
        // __x
        let board = Board {
            fields: vec![
                Field::X,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::X,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::X,
            ],
        };
        assert_eq!(board.has_won_player(Field::X), true);
    }

    #[test]
    fn test_diagonal2() {
        // __x
        // _x_
        // x__
        let board = Board {
            fields: vec![
                Field::Free,
                Field::Free,
                Field::X,
                Field::Free,
                Field::X,
                Field::Free,
                Field::X,
                Field::Free,
                Field::Free,
            ],
        };
        assert_eq!(board.has_won_player(Field::X), true);
    }

    #[test]
    fn test_check_false1() {
        // _xx
        // _x_
        // __x
        let board = Board {
            fields: vec![
                Field::Free,
                Field::X,
                Field::X,
                Field::Free,
                Field::X,
                Field::Free,
                Field::Free,
                Field::Free,
                Field::X,
            ],
        };
        assert_eq!(board.has_won_player(Field::X), false);
    }

    #[test]
    fn test_check_false2() {
        // xx_
        // _xx
        // x__
        let board = Board {
            fields: vec![
                Field::X,
                Field::X,
                Field::Free,
                Field::Free,
                Field::X,
                Field::X,
                Field::X,
                Field::Free,
                Field::Free,
            ],
        };
        assert_eq!(board.has_won_player(Field::X), false);
    }
}
