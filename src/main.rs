pub mod display;
pub mod moves;
pub mod solver;

use crate::moves::Move;
use crate::solver::Solver;

fn main() {
    let mut cube = Cube::default();
    assert!(cube.is_solved());
    assert_eq!(Cube::evaluate(&cube), 1.0);

    // scramble the cube with random moves
    let scramble_moves = 22u32;
    for _ in 0..scramble_moves {
        let rand = rand::random_range(1u8..=6);
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

    assert!(
        (cube.is_edge_valid() == cube.is_corner_valid())
            && (cube.corner_parity() == cube.edge_parity()),
        "Cube not valid after {} scrambles: {}",
        scramble_moves,
        cube
    );

    let max_depth = 18;
    println!("Cube state after scramble:\n{}", cube);
    println!("Solving with max depth {}", max_depth);
    let cube_copy = cube;
    let start = std::time::Instant::now();
    let result = Solver::init(cube_copy, max_depth).solve();
    match result.solution {
        Some(solution) => {
            println!(
                "Solution found in {} moves, node searched: {}",
                solution.len(),
                result.nodes_searched
            );
            println!("Time taken: {:?}", start.elapsed());
            for m in &solution {
                print!("{} ", m);
            }
            println!();
            for m in &solution {
                cube.apply_move(*m);
            }
            assert!(
                (cube.is_edge_valid() == cube.is_corner_valid())
                    && (cube.corner_parity() == cube.edge_parity()),
                "Cube not valid after solution: {}",
                scramble_moves,
            );
            println!("Cube state after solution:\n{}", cube);
        }
        None => {
            println!("No solution found within {} moves.", max_depth);
        }
    }
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

    #[inline(always)]
    fn evaluate(cube: &Cube) -> f32 {
        let mut correct_cubies = 0u32;

        // check corners for their correct position and orientation
        for i in 0..8 {
            if cube.corner_permutation[i] == i as u8 && cube.corner_orientation[i] == 0 {
                correct_cubies += 1;
            }
        }

        // check edge similarly
        for i in 0..12 {
            if cube.edge_permutation[i] == i as u8 && cube.edge_orientation[i] == 0 {
                correct_cubies += 1;
            }
        }

        correct_cubies as f32 / 20.0 // 0..1
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
