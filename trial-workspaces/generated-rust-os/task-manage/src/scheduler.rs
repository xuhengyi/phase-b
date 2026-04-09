/// Schedule trait defines the minimal interface of a task scheduler.
pub trait Schedule<I> {
    fn add(&mut self, id: I);
    fn fetch(&mut self) -> Option<I>;
}
