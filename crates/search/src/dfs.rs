use crate::traits::{Problem, Strategy};

pub struct Dfs<P>(pub Vec<P>);

impl<T> Default for Dfs<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<P: Problem> Strategy<P> for Dfs<<P as Problem>::State> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push(&mut self, value: <P as Problem>::State) {
        self.0.push(value)
    }

    fn pop(&mut self) -> Option<<P as Problem>::State> {
        self.0.pop()
    }
}
