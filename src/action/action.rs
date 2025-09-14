pub trait Action {
    fn run(&mut self) -> Result<(), String>;
    fn undo(&mut self) -> Result<(), String>;
}
