use std::collections::VecDeque;

use crate::traits::{Problem, Strategy};

pub struct Bfs<P>(pub VecDeque<P>);

impl<T> Default for Bfs<T> {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

impl<P: Problem> Strategy<P> for Bfs<<P as Problem>::State> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push(&mut self, value: <P as Problem>::State) {
        self.0.push_back(value)
    }

    fn pop(&mut self) -> Option<<P as Problem>::State> {
        self.0.pop_front()
    }
}
