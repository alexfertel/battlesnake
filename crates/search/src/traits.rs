pub trait Problem {
    type State: Default + core::fmt::Debug;

    fn get_start(&self) -> Self::State;
    fn get_successors(&mut self, state: &Self::State) -> Vec<Self::State>;
    fn is_end(&self, state: &Self::State) -> bool;
}

pub trait Strategy<P: Problem> {
    fn is_empty(&self) -> bool;
    fn push(&mut self, value: P::State);
    fn pop(&mut self) -> Option<P::State>;
}
