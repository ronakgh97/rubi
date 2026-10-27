#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
enum Corner {
    URF = 0, UFL = 1, ULB = 2, UBR = 3,
    DFR = 4, DLF = 5, DBL = 6, DRB = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
enum Edge {
    UR = 0, UF = 1, UL = 2, UB = 3,
    DR = 4, DF = 5, DL = 6, DB = 7,
    FR = 8, FL = 9, BL = 10, BR = 11,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub corner_from: [u8; 8],
    pub corner_delta: [u8; 8],
    pub edge_from: [u8; 12],
    pub edge_delta: [u8; 12],
}

impl Move {
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
    pub const fn PRIME(self) -> Self {
        let mut inverse = self;
        let mut i = 0;

        while i < 8 {
            let from = self.corner_from[i] as usize;
            inverse.corner_from[from] = i as u8;
            inverse.corner_delta[from] = (3 - self.corner_delta[i]) % 3;
            i += 1;
        }

        i = 0;
        while i < 12 {
            let from = self.edge_from[i] as usize;
            inverse.edge_from[from] = i as u8;
            inverse.edge_delta[from] = (2 - self.edge_delta[i]) % 2;
            i += 1;
        }

        inverse
    }
    #[allow(non_snake_case)]
    pub const fn DOUBLE(self) -> Self {
        let mut double = self;
        let mut i = 0;

        while i < 8 {
            let from = self.corner_from[i] as usize;
            double.corner_from[i] = self.corner_from[from];
            double.corner_delta[i] = (self.corner_delta[from] + self.corner_delta[i]) % 3;
            i += 1;
        }

        i = 0;
        while i < 12 {
            let from = self.edge_from[i] as usize;
            double.edge_from[i] = self.edge_from[from];
            double.edge_delta[i] = (self.edge_delta[from] + self.edge_delta[i]) % 2;
            i += 1;
        }

        double
    }
}

#[allow(non_snake_case)]
/// Format: U, U', U2, D, D', D2, L, L', L2, R, R', R2, F, F', F2, B, B', B2
pub const fn ALL_MOVES() -> [Move; 18] {
    let f = Move::F();
    let b = Move::B();
    let u = Move::U();
    let d = Move::D();
    let l = Move::L();
    let r = Move::R();

    [
        u,
        u.PRIME(),
        u.DOUBLE(),
        d,
        d.PRIME(),
        d.DOUBLE(),
        l,
        l.PRIME(),
        l.DOUBLE(),
        r,
        r.PRIME(),
        r.DOUBLE(),
        f,
        f.PRIME(),
        f.DOUBLE(),
        b,
        b.PRIME(),
        b.DOUBLE(),
    ]
}
