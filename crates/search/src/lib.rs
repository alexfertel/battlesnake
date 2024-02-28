use traits::{Problem, Strategy};

mod bfs;
mod dfs;
mod direction;
mod dungeon;
mod point;
mod traits;

pub fn search<P: Problem, S: Strategy<P>>(mut problem: P, mut strategy: S) -> P::State {
    let start = problem.get_start();
    strategy.push(start);

    while let Some(current) = strategy.pop() {
        if problem.is_end(&current) {
            return current;
        }

        let succs = problem.get_successors(&current);
        for succ in succs {
            strategy.push(succ);
        }
    }

    problem.get_start()
}
