pub mod display;
pub mod moves;
pub mod solver;

use crate::moves::Move;

fn main() {
    let mut cube = Cube::default();
    assert!(cube.is_solved());

    // scramble the cube with random moves
    for _ in 0..443665 {
        let rand = rand::random_range(0..6);
        match rand {
            1 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::B());
                } else {
                    cube.apply_move(Move::B().PRIME());
                }
            }
            2 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::U());
                } else {
                    cube.apply_move(Move::U().PRIME());
                }
            }
            3 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::L());
                } else {
                    cube.apply_move(Move::L().PRIME());
                }
            }
            4 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::R());
                } else {
                    cube.apply_move(Move::R().PRIME());
                }
            }
            5 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::F());
                } else {
                    cube.apply_move(Move::F().PRIME());
                }
            }
            6 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::D());
                } else {
                    cube.apply_move(Move::D().PRIME());
                }
            }
            _ => {}
        }
    }

    println!(
        "Is cube valid after scrambled? {}",
        (cube.is_edge_valid() == cube.is_corner_valid())
            && (cube.corner_parity() == cube.edge_parity())
    );
    println!("Cube state after scramble:\n{}", cube);
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube {
    corner_permutation: [u8; 8],
    corner_orientation: [u8; 8],
    edge_permutation: [u8; 12],
    edge_orientation: [u8; 12],
    // and other 6 fixed faces
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            corner_permutation: [0, 1, 2, 3, 4, 5, 6, 7],
            corner_orientation: [0; 8],
            edge_permutation: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            edge_orientation: [0; 12],
        }
    }
}

impl Cube {
    #[inline(always)]
    pub fn apply_move(&mut self, r#move: Move) {
        // get old state
        let old_cp = self.corner_permutation;
        let old_co = self.corner_orientation;
        let old_ep = self.edge_permutation;
        let old_eo = self.edge_orientation;

        // apply change to corners
        for i in 0..8 {
            // get old corner state
            let from = r#move.corner_from[i] as usize;
            self.corner_permutation[i] = old_cp[from]; // apply new state
            self.corner_orientation[i] = (old_co[from] + r#move.corner_delta[i]) % 3; // update net orientation
        }

        // apply change to edges (same as corners)
        for i in 0..12 {
            let from = r#move.edge_from[i] as usize;
            self.edge_permutation[i] = old_ep[from];
            self.edge_orientation[i] = (old_eo[from] + r#move.edge_delta[i]) % 2;
        }
    }

    #[inline(always)]
    fn is_solved(&self) -> bool {
        let cube = Cube::default();
        self == &cube
    }

    #[inline]
    fn is_corner_valid(&self) -> bool {
        let mut corner_orientation_sum = 0;
        for &orientation in &self.corner_orientation {
            corner_orientation_sum += orientation as u32;
        }
        corner_orientation_sum.is_multiple_of(3)
    }

    #[inline]
    fn is_edge_valid(&self) -> bool {
        let mut edge_orientation_sum = 0;
        for &orientation in &self.edge_orientation {
            edge_orientation_sum += orientation as u32;
        }
        edge_orientation_sum.is_multiple_of(2)
    }

    #[inline]
    fn corner_parity(&self) -> bool {
        let mut corner_permutation_parity = 0;
        for i in 0..7 {
            for j in (i + 1)..8 {
                if self.corner_permutation[i] > self.corner_permutation[j] {
                    corner_permutation_parity ^= 1;
                }
            }
        }
        corner_permutation_parity == 0
    }

    #[inline]
    fn edge_parity(&self) -> bool {
        let mut edge_permutation_parity = 0;
        for i in 0..11 {
            for j in (i + 1)..12 {
                if self.edge_permutation[i] > self.edge_permutation[j] {
                    edge_permutation_parity ^= 1;
                }
            }
        }
        edge_permutation_parity == 0
    }
}
