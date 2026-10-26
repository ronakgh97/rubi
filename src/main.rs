pub mod moves;
pub mod solver;

use crate::moves::Move;
use std::fmt::Display;

fn main() {
    let mut cube = Cube::default();

    // scramble the cube with random moves
    for _ in 0..443665 {
        let rand = rand::random_range(0..6);
        match rand {
            1 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::B());
                } else {
                    cube.apply_move(Move::B().prime());
                }
            }
            2 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::U());
                } else {
                    cube.apply_move(Move::U().prime());
                }
            }
            3 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::L());
                } else {
                    cube.apply_move(Move::L().prime());
                }
            }
            4 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::R());
                } else {
                    cube.apply_move(Move::R().prime());
                }
            }
            5 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::F());
                } else {
                    cube.apply_move(Move::F().prime());
                }
            }
            6 => {
                if rand::random_bool(0.5) {
                    cube.apply_move(Move::D());
                } else {
                    cube.apply_move(Move::D().prime());
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
struct Cube {
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

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let front_face = [
            self.edge_permutation[1],
            self.edge_permutation[8],
            self.edge_permutation[5],
            self.edge_permutation[9],
        ];
        let back_face = [
            self.edge_permutation[3],
            self.edge_permutation[10],
            self.edge_permutation[7],
            self.edge_permutation[11],
        ];
        let up_face = [
            self.edge_permutation[0],
            self.edge_permutation[1],
            self.edge_permutation[2],
            self.edge_permutation[3],
        ];
        let down_face = [
            self.edge_permutation[4],
            self.edge_permutation[5],
            self.edge_permutation[6],
            self.edge_permutation[7],
        ];
        let left_face = [
            self.edge_permutation[2],
            self.edge_permutation[9],
            self.edge_permutation[6],
            self.edge_permutation[10],
        ];
        let right_face = [
            self.edge_permutation[0],
            self.edge_permutation[7],
            self.edge_permutation[4],
            self.edge_permutation[11],
        ];
        // TODO; add colors somehow maybe
        write!(
            f,
            "Front face: {:?}\nBack face: {:?}\nUp face: {:?}\nDown face: {:?}\nLeft face: {:?}\nRight face: {:?}",
            front_face, back_face, up_face, down_face, left_face, right_face
        )
    }
}

impl Cube {
    #[inline]
    fn apply_move(&mut self, r#move: Move) {
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

mod test {
    #[test]
    fn test() {
        todo!()
    }
}
