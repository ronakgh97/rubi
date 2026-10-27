use crate::Cube;
use colored::{Color, Colorize};
use std::fmt;
use std::fmt::{Display, Write};

const U: usize = 0;
const R: usize = 1;
const F: usize = 2;
const D: usize = 3;
const L: usize = 4;
const B: usize = 5;

const CORNER_FACELETS: [[(usize, usize); 3]; 8] = [
    [(U, 8), (R, 0), (F, 2)], // URF
    [(U, 6), (F, 0), (L, 2)], // UFL
    [(U, 0), (L, 0), (B, 2)], // ULB
    [(U, 2), (B, 0), (R, 2)], // UBR
    [(D, 2), (F, 8), (R, 6)], // DFR
    [(D, 0), (L, 8), (F, 6)], // DLF
    [(D, 6), (B, 8), (L, 6)], // DBL
    [(D, 8), (R, 8), (B, 6)], // DRB
];

const EDGE_FACELETS: [[(usize, usize); 2]; 12] = [
    [(U, 5), (R, 1)], // UR
    [(U, 7), (F, 1)], // UF
    [(U, 3), (L, 1)], // UL
    [(U, 1), (B, 1)], // UB
    [(D, 5), (R, 7)], // DR
    [(D, 1), (F, 7)], // DF
    [(D, 3), (L, 7)], // DL
    [(D, 7), (B, 7)], // DB
    [(F, 5), (R, 3)], // FR
    [(F, 3), (L, 5)], // FL
    [(B, 5), (L, 3)], // BL
    [(B, 3), (R, 5)], // BR
];

const LETTERS: [&str; 6] = [" W ", " R ", " G ", " Y ", " O ", " B "];
const COLORS: [Color; 6] = [
    Color::BrightWhite,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::TrueColor {
        // Orange
        r: 252,
        g: 102,
        b: 0,
    },
    Color::Blue,
];

impl Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut faces = [[U; 9], [R; 9], [F; 9], [D; 9], [L; 9], [B; 9]];

        // place each corner's three stickers, with their twists
        for (position, _) in CORNER_FACELETS.iter().enumerate() {
            let piece = self.corner_permutation[position] as usize;
            let twist = self.corner_orientation[position] as usize;

            for (sticker, _) in CORNER_FACELETS.iter().enumerate().take(3) {
                let destination = (sticker + twist) % 3;

                let (face, cell) = CORNER_FACELETS[position][destination];
                let (original_face, _) = CORNER_FACELETS[piece][sticker];

                faces[face][cell] = original_face;
            }
        }

        // place each edge's two stickers, with their flips
        for (position, _) in EDGE_FACELETS.iter().enumerate() {
            let piece = self.edge_permutation[position] as usize;
            let flip = self.edge_orientation[position] as usize;

            for (sticker, _) in EDGE_FACELETS.iter().enumerate().take(2) {
                let destination = (sticker + flip) % 2;

                let (face, cell) = EDGE_FACELETS[position][destination];
                let (original_face, _) = EDGE_FACELETS[piece][sticker];

                faces[face][cell] = original_face;
            }
        }

        // convert each face to a grid of letters (colored rows)
        let mut face_rows: Vec<Vec<String>> = Vec::with_capacity(6);

        for face in &faces {
            let mut rows = Vec::with_capacity(3);
            for row in 0..3 {
                let mut line = String::with_capacity(9);
                for column in 0..3 {
                    let color = face[row * 3 + column];

                    write!(
                        line,
                        "{}",
                        LETTERS[color]
                            .color(Color::BrightWhite)
                            .on_color(COLORS[color])
                    )?;
                }
                rows.push(line);
            }
            face_rows.push(rows);
        }

        //           U
        //       L   F   R   B
        //           D
        writeln!(f, "               U")?;
        #[allow(clippy::needless_range_loop)]
        for row in 0..3 {
            writeln!(f, "           {}", face_rows[U][row])?;
        }

        writeln!(f, "\n    L          F          R          B")?;
        #[allow(clippy::needless_range_loop)]
        for row in 0..3 {
            writeln!(
                f,
                "{}  {}  {}  {}",
                face_rows[L][row], face_rows[F][row], face_rows[R][row], face_rows[B][row],
            )?;
        }

        writeln!(f, "\n               D")?;
        #[allow(clippy::needless_range_loop)]
        for row in 0..3 {
            write!(f, "           {}", face_rows[D][row])?;

            // for new line after each row (except last)
            if row < 2 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
