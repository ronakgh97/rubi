fn main() {
    let cube = Cube::default();
    println!(
        "Is cube valid? {}",
        cube.is_corner_valid()
            && cube.is_edge_valid()
            && (cube.corner_parity() == cube.edge_parity())
    );
}

// I OFTEN FORGET/MISS, THIS IS SO FUCKING STUPID AND RETARD MAXXING
// U = UPPER
// D = DOWN
// F = FRONT
// B = BACK
// L = LEFT
// R = RIGHT

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
enum Corner {
    URF = 0, UFL = 1, ULB = 2, UBR = 3,
    DFR = 4, DLF = 5, DBL = 6, DRB = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
enum Edge {
    UR = 0, UF = 1, UL = 2, UB = 3,
    DR = 4, DF = 5, DL = 6, DB = 7,
    FR = 8, FL = 9, BL = 10, BR = 11,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Move {
    corner_from: [u8; 8],
    corner_delta: [u8; 8],
    edge_from: [u8; 12],
    edge_delta: [u8; 12],
}

impl Move {
    pub const fn R() -> Self {}
    pub const fn L() -> Self {}
    pub const fn U() -> Self {}
    pub const fn D() -> Self {}
    pub const fn F() -> Self {}
    pub const fn B() -> Self {}

    pub fn prime(&self) -> Self {}
    pub fn double(&self) -> Self {}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    corner_permutation: [u8; 8],
    corner_orientation: [u8; 8],
    edge_permutation: [u8; 12],
    edge_orientation: [u8; 12],
    // 6 fixed faces
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
        corner_orientation_sum % 3 == 0
    }

    #[inline]
    fn is_edge_valid(&self) -> bool {
        let mut edge_orientation_sum = 0;
        for &orientation in &self.edge_orientation {
            edge_orientation_sum += orientation as u32;
        }
        edge_orientation_sum % 2 == 0
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
