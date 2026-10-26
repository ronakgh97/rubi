mod moves;

fn main() {
    let cube = Cube::default();
    println!(
        "Is cube valid? {}",
        cube.is_corner_valid()
            && cube.is_edge_valid()
            && (cube.corner_parity() == cube.edge_parity())
    );
}

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
    #[allow(non_snake_case)]
    pub const fn R() -> Self {
        Move {
            // corners: cycle URF -> UBR -> DRB -> DFR -> URF
            corner_from: [
                Corner::DFR as u8, // URF <- DFR
                Corner::UFL as u8,
                Corner::ULB as u8,
                Corner::URF as u8, // UBR <- URF
                Corner::DRB as u8, // DFR <- DRB
                Corner::DLF as u8,
                Corner::DBL as u8,
                Corner::UBR as u8, // DRB <- UBR
            ],
            corner_delta: [2, 0, 0, 1, 1, 0, 0, 2],
            // edges: cycle UR -> BR -> DR -> FR -> UR
            edge_from: [
                Edge::FR as u8, // UR <- FR
                Edge::UF as u8,
                Edge::UL as u8,
                Edge::UB as u8,
                Edge::BR as u8, // DR <- BR
                Edge::DF as u8,
                Edge::DL as u8,
                Edge::DB as u8,
                Edge::DR as u8, // FR <- DR
                Edge::FL as u8,
                Edge::BL as u8,
                Edge::UR as u8, // BR <- UR
            ],
            edge_delta: [0; 12],
        }
    }
    #[allow(non_snake_case)]
    pub const fn L() -> Self {
        Self {
            // corners: cycle UFL -> DLF -> DBL -> ULB -> UFL
            corner_from: [
                Corner::URF as u8,
                Corner::ULB as u8, // UFL <- ULB
                Corner::DBL as u8, // ULB <- DBL
                Corner::UBR as u8,
                Corner::DFR as u8,
                Corner::UFL as u8, // DLF <- UFL
                Corner::DLF as u8, // DBL <- DLF
                Corner::DRB as u8,
            ],
            corner_delta: [0, 1, 2, 0, 0, 2, 1, 0],
            // edges: cycle UL -> FL -> DL -> BL -> UL
            edge_from: [
                Edge::UR as u8,
                Edge::UF as u8,
                Edge::BL as u8, // UL <- BL
                Edge::UB as u8,
                Edge::DR as u8,
                Edge::DF as u8,
                Edge::FL as u8, // DL <- FL
                Edge::DB as u8,
                Edge::FR as u8,
                Edge::UL as u8, // FL <- UL
                Edge::DL as u8, // BL <- DL
                Edge::BR as u8,
            ],
            edge_delta: [0; 12],
        }
    }
    #[allow(non_snake_case)]
    pub const fn U() -> Self {
        Self {
            // corners: cycle URF -> UFL -> ULB -> UBR -> URF
            corner_from: [
                Corner::UBR as u8, // URF <- UBR
                Corner::URF as u8, // UFL <- URF
                Corner::UFL as u8, // ULB <- UFL
                Corner::ULB as u8, // UBR <- ULB
                Corner::DFR as u8,
                Corner::DLF as u8,
                Corner::DBL as u8,
                Corner::DRB as u8,
            ],
            corner_delta: [0; 8],
            // edges: cycle UR -> UF -> UL -> UB -> UR
            edge_from: [
                Edge::UB as u8, // UR <- UB
                Edge::UR as u8, // UF <- UR
                Edge::UF as u8, // UL <- UF
                Edge::UL as u8, // UB <- UL
                Edge::DR as u8,
                Edge::DF as u8,
                Edge::DL as u8,
                Edge::DB as u8,
                Edge::FR as u8,
                Edge::FL as u8,
                Edge::BL as u8,
                Edge::BR as u8,
            ],
            edge_delta: [0; 12],
        }
    }
    #[allow(non_snake_case)]
    pub const fn D() -> Self {
        Self {
            // corners: cycle DFR -> DRB -> DBL -> DLF -> DFR
            corner_from: [
                Corner::URF as u8,
                Corner::UFL as u8,
                Corner::ULB as u8,
                Corner::UBR as u8,
                Corner::DLF as u8, // DFR <- DLF
                Corner::DBL as u8, // DLF <- DBL
                Corner::DRB as u8, // DBL <- DRB
                Corner::DFR as u8, // DRB <- DFR
            ],
            corner_delta: [0; 8],
            // edges: cycle DR -> DB -> DL -> DF -> DR
            edge_from: [
                Edge::UR as u8,
                Edge::UF as u8,
                Edge::UL as u8,
                Edge::UB as u8,
                Edge::DF as u8, // DR <- DF
                Edge::DL as u8, // DF <- DL
                Edge::DB as u8, // DL <- DB
                Edge::DR as u8, // DB <- DR
                Edge::FR as u8,
                Edge::FL as u8,
                Edge::BL as u8,
                Edge::BR as u8,
            ],
            edge_delta: [0; 12],
        }
    }
    #[allow(non_snake_case)]
    pub const fn F() -> Self {
        Self {
            // corners: cycle URF -> DFR -> DLF -> UFL -> URF
            corner_from: [
                Corner::UFL as u8, // URF <- UFL
                Corner::DLF as u8, // UFL <- DLF
                Corner::ULB as u8,
                Corner::UBR as u8,
                Corner::URF as u8, // DFR <- URF
                Corner::DFR as u8, // DLF <- DFR
                Corner::DBL as u8,
                Corner::DRB as u8,
            ],
            corner_delta: [1, 2, 0, 0, 2, 1, 0, 0],
            // edges: cycle UF -> FR -> DF -> FL -> UF
            edge_from: [
                Edge::UR as u8,
                Edge::FL as u8, // UF <- FL
                Edge::UL as u8,
                Edge::UB as u8,
                Edge::DR as u8,
                Edge::FR as u8, // DF <- FR
                Edge::DL as u8,
                Edge::DB as u8,
                Edge::UF as u8, // FR <- UF
                Edge::DF as u8, // FL <- DF
                Edge::BL as u8,
                Edge::BR as u8,
            ],
            edge_delta: [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
        }
    }
    #[allow(non_snake_case)]
    pub const fn B() -> Self {
        Self {
            // corners: cycle UBR -> ULB -> DBL -> DRB -> UBR
            corner_from: [
                Corner::URF as u8,
                Corner::UFL as u8,
                Corner::UBR as u8, // ULB <- UBR
                Corner::DRB as u8, // UBR <- DRB
                Corner::DFR as u8,
                Corner::DLF as u8,
                Corner::ULB as u8, // DBL <- ULB
                Corner::DBL as u8, // DRB <- DBL
            ],
            corner_delta: [0, 0, 1, 2, 0, 0, 2, 1],
            // edges: cycle UB -> BL -> DB -> BR -> UB
            edge_from: [
                Edge::UR as u8,
                Edge::UF as u8,
                Edge::UL as u8,
                Edge::BR as u8, // UB <- BR
                Edge::DR as u8,
                Edge::DF as u8,
                Edge::DL as u8,
                Edge::BL as u8, // DB <- BL
                Edge::FR as u8,
                Edge::FL as u8,
                Edge::UB as u8, // BL <- UB
                Edge::DB as u8, // BR <- DB
            ],
            edge_delta: [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1],
        }
    }
    pub fn prime(&self) -> Self {
        let mut inverse = *self;
        for i in 0..8 {
            let from = self.corner_from[i] as usize;
            inverse.corner_from[from] = i as u8;
            inverse.corner_delta[from] = (3 - self.corner_delta[i]) % 3;
        }
        for i in 0..12 {
            let from = self.edge_from[i] as usize;
            inverse.edge_from[from] = i as u8;
            inverse.edge_delta[from] = (2 - self.edge_delta[i]) % 2;
        }
        inverse
    }
    pub fn double(&self) -> Self {
        let mut double = *self;
        for i in 0..8 {
            let from = self.corner_from[i] as usize;
            double.corner_from[i] = self.corner_from[from];
            double.corner_delta[i] = (self.corner_delta[from] + self.corner_delta[i]) % 3;
        }
        for i in 0..12 {
            let from = self.edge_from[i] as usize;
            double.edge_from[i] = self.edge_from[from];
            double.edge_delta[i] = (self.edge_delta[from] + self.edge_delta[i]) % 2;
        }
        double
    }
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
