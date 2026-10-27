use parallelos::finish_first;
use parallelos::pool::WorkerPool;
use parallelos::tasks::TaskCost;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
// TODO; Native implementation for now
use crate::Cube;
use crate::Move;
use crate::moves::ALL_MOVES;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MoveEntry {
    face: u8,
    opposite_face: u8,
    r#move: Move,
}
fn move_entry_list() -> [MoveEntry; 18] {
    let moves = ALL_MOVES();
    core::array::from_fn(|i| MoveEntry {
        // NOTE: This works only because the moves
        // are declared by that order in the ALL_MOVES() function
        face: (i / 3) as u8,
        // ALL_MOVES order is U,D,L,R,F,B -> opposites are adjacent indices (UD, LR, FB)
        opposite_face: ((i / 3) ^ 1) as u8,
        r#move: moves[i],
    })
}

pub struct Solver {
    cube: Cube,
    node_searched: u64,
    max_depth: usize,
    workers: WorkerPool,
    move_entries: [MoveEntry; 18],
}

pub struct SolveResult {
    pub solution: Option<Vec<Move>>,
    pub nodes_searched: u64,
    pub depth_reached: usize,
}

impl Solver {
    pub fn init(cube: Cube, max_depth: usize) -> Self {
        let move_entries = move_entry_list();
        Self {
            cube,
            node_searched: 0,
            max_depth,
            workers: WorkerPool::init_with(move_entries.len())
                .expect("failed to initialize worker pool"),
            move_entries,
        }
    }
    pub fn solve(&mut self) -> SolveResult {
        if self.cube.is_solved() {
            return SolveResult {
                solution: Some(vec![]),
                nodes_searched: 0,
                depth_reached: 0,
            };
        }

        let moves_entries = Arc::new(self.move_entries);
        let nodes_searched = Arc::new(AtomicU64::new(0));
        for depth in 1..=self.max_depth {
            let mut task_handles = Vec::with_capacity(18);

            for (i, mv) in moves_entries.iter().enumerate() {
                let mut cube_copy = self.cube;
                let mut moves_traversed = Vec::with_capacity(depth);

                cube_copy.apply_move(mv.r#move);
                moves_traversed.push(mv.r#move);

                let moves_entries_task_clone = Arc::clone(&moves_entries);
                let nodes_searched_task_clone = Arc::clone(&nodes_searched);
                let initial_move = *mv;

                let handle = self
                    .workers
                    .submit_to_worker(
                        i,
                        move || {
                            let mut local_nodes = 0;

                            let found = dfs(
                                &mut cube_copy,
                                depth.saturating_sub(1),
                                &mut local_nodes,
                                &mut moves_traversed,
                                Some(&initial_move),
                                &moves_entries_task_clone[..],
                            );

                            nodes_searched_task_clone.fetch_add(local_nodes, Ordering::Relaxed);

                            (found, moves_traversed)
                        },
                        TaskCost::Normal,
                    )
                    .expect("failed to submit dfs task to worker");

                task_handles.push(handle);
            }

            // finish_first returns tasks in *completion* order, not success order.
            // Keep polling until one branch reports found == true, or all are exhausted.
            let mut handles = task_handles;
            let mut solved: Option<Vec<Move>> = None;

            while !handles.is_empty() {
                let (res, remaining) = finish_first(handles);
                handles = remaining;
                let ((found, moves), _worker_id) = res.expect("task panicked or failed to send");
                if found {
                    solved = Some(moves);
                    break;
                }
                // this branch was a dead end -> poll the next finished task
            }

            // handles is empty here unless we broke early (all branches drained)
            self.node_searched = nodes_searched.load(Ordering::Relaxed);

            if let Some(path) = solved {
                return SolveResult {
                    solution: Some(path),
                    nodes_searched: self.node_searched,
                    depth_reached: depth,
                };
            }
        }

        SolveResult {
            solution: None,
            nodes_searched: self.node_searched,
            depth_reached: self.max_depth,
        }
    }
}

fn dfs(
    cube: &mut Cube,
    depth_remaining: usize,
    nodes_searched: &mut u64,
    moves_tracked: &mut Vec<Move>,
    previous_move_entry: Option<&MoveEntry>,
    move_entries: &[MoveEntry],
) -> bool {
    *nodes_searched += 1;

    if cube.is_solved() {
        return true;
    }
    if depth_remaining == 0 {
        return false;
    }

    for mv in move_entries {
        if let Some(prev_move) = previous_move_entry {
            // don't turn the same face twice in a row
            if prev_move.face == mv.face {
                continue;
            }
            // don't turn the opposite face if
            // the previous move was turned off the current face in same direction
            if mv.face == prev_move.opposite_face && mv.face > prev_move.face {
                continue;
            }
        }

        let previous_cube = *cube;
        cube.apply_move(mv.r#move);
        moves_tracked.push(mv.r#move);

        if dfs(
            cube,
            depth_remaining - 1,
            nodes_searched,
            moves_tracked,
            Some(mv),
            move_entries,
        ) {
            return true;
        }
        moves_tracked.pop();
        *cube = previous_cube; // backtrack
    }
    false
}
