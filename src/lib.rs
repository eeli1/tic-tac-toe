mod utils;

use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
#[derive(PartialEq, Debug, Clone)]
struct Board {
    fields: Vec<Field>,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        Board {
            fields: vec![Field::Free; 9],
        }
    }

    fn get(&self, x: usize, y: usize) -> Field {
        self.fields[3 * y + x].clone()
    }

    fn set(&mut self, x: usize, y: usize, value: Field) {
        self.fields[3 * y + x] = value;
    }

    pub fn make_move(&mut self, x: usize, y: usize, player: Field) -> bool {
        if self.get(x, y) != Field::Free {
            return false;
        }
        self.set(x, y, player);
        true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
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
        // ..x
        // .x.
        // x..
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
        // .xx
        // .x.
        // ..x
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
        // xx.
        // .xx
        // x..
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
