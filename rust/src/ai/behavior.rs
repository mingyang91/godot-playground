pub trait Behavior<I, A> {
    fn update(&mut self, info: I, delta: f64);
    fn action(&self) -> A;
}
