pub trait SourceEvent<T> {
    fn apply(&self, t: &mut T);
}

pub trait EventSourced<T> {
    fn get_events(self) -> Vec<Box<dyn SourceEvent<T>>>;
    fn apply(&mut self, ev: impl SourceEvent<T>);
}
