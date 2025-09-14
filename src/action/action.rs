pub trait Action {
    fn run(&mut self);
    fn undo(&mut self);
}
