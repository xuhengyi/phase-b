/// Manage trait abstracts a task container that can insert/delete/fetch items by id.
pub trait Manage<T, I> {
    fn insert(&mut self, id: I, item: T);
    fn delete(&mut self, id: I);
    fn get_mut(&mut self, id: I) -> Option<&mut T>;
}
