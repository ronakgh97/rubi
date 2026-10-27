// TODO; Native implementation for now
use crate::Cube;
use crate::Move;
use crate::moves::ALL_MOVES;

#[derive(Debug)]
pub struct SearchResult {
    pub solution: Option<Vec<Move>>,
    pub nodes_searched: u64,
    pub depth_reached: usize,
}

struct SearchMove {
    face: u8,
    opposite_face: u8,
    r#move: Move,
}
fn search_moves_list() -> [SearchMove; 18] {
    let moves = ALL_MOVES();
    core::array::from_fn(|i| SearchMove {
        // NOTE: This works only because the moves
        // are declared by that order in the ALL_MOVES() function
        face: (i / 3) as u8,
        // ALL_MOVES order is U,D,L,R,F,B → opposites are adjacent indices (U↔D, L↔R, F↔B)
        opposite_face: ((i / 3) ^ 1) as u8,
        r#move: moves[i],
    })
}

pub fn iddfs(cube: Cube, max_depth: usize) -> SearchResult {
    let mut nodes_searched = 0;
    if cube.is_solved() {
        return SearchResult {
            solution: Some(Vec::with_capacity(20)),
            nodes_searched,
            depth_reached: 0,
        };
    }

    let search_moves_list = search_moves_list();
    for depth in 1..=max_depth {
        let mut cube_copy = cube;
        let mut move_path = Vec::with_capacity(depth);

        if dfs(
            &mut cube_copy,
            &mut move_path,
            None,
            &search_moves_list,
            depth,
            &mut nodes_searched,
        ) {
            return SearchResult {
                solution: Some(move_path),
                nodes_searched,
                depth_reached: depth,
            };
        }
    }

    SearchResult {
        solution: None,
        nodes_searched,
        depth_reached: max_depth,
    }
}

fn dfs(
    cube: &mut Cube,
    moves_path: &mut Vec<Move>,
    previous_search_move: Option<&SearchMove>,
    search_moves_list: &[SearchMove],
    depth_remaining: usize,
    nodes_searched: &mut u64,
) -> bool {
    *nodes_searched += 1;

    if cube.is_solved() {
        return true;
    }
    if depth_remaining == 0 {
        return false;
    }

    for mv in search_moves_list {
        if let Some(prev_move) = previous_search_move {
            // don't turn the same face twice in a row
            if prev_move.face == mv.face {
                continue;
            }
            // don't turn the opposite face if
            // the previous move was turned of the current face in same direction
            if mv.face == prev_move.opposite_face && mv.face > prev_move.face {
                continue;
            }
        }

        let previous_cube = *cube;
        cube.apply_move(mv.r#move);
        moves_path.push(mv.r#move);

        if dfs(
            cube,
            moves_path,
            Some(mv),
            search_moves_list,
            depth_remaining - 1,
            nodes_searched,
        ) {
            return true;
        }

        moves_path.pop();
        *cube = previous_cube;
    }

    false
}
