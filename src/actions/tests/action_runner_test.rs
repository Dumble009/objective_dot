#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::action::Action;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct MockAction {
        run_called_count: Option<Rc<RefCell<i32>>>,
        undo_called_count: Option<Rc<RefCell<i32>>>,
    }

    impl Action for MockAction {
        fn run(&mut self) -> Result<(), String> {
            if let Some(ref count) = self.run_called_count {
                *count.borrow_mut() += 1;
            }
            Ok(())
        }
        fn undo(&mut self) -> Result<(), String> {
            if let Some(ref count) = self.undo_called_count {
                *count.borrow_mut() += 1;
            }
            Ok(())
        }
    }

    impl MockAction {
        pub fn new(r: Rc<RefCell<i32>>, u: Rc<RefCell<i32>>) -> Self {
            MockAction {
                run_called_count: Some(r),
                undo_called_count: Some(u),
            }
        }
    }

    #[test]
    fn run_do_and_undo_test() {
        let mut action_runner = ActionRunner::new();
        let run_called_count1 = Rc::new(RefCell::new(0));
        let undo_called_count1 = Rc::new(RefCell::new(0));
        let run_called_count2 = Rc::new(RefCell::new(0));
        let undo_called_count2 = Rc::new(RefCell::new(0));
        let action1 = Box::new(MockAction::new(
            run_called_count1.clone(),
            undo_called_count1.clone(),
        ));
        let action2 = Box::new(MockAction::new(
            run_called_count2.clone(),
            undo_called_count2.clone(),
        ));

        let res = action_runner.run_action(action1);
        assert!(res.is_ok());
        assert_eq!(1, *run_called_count1.borrow());
        assert_eq!(0, *run_called_count2.borrow());

        let res = action_runner.run_action(action2);
        assert!(res.is_ok());
        assert_eq!(1, *run_called_count1.borrow());
        assert_eq!(1, *run_called_count2.borrow());

        // undo は run と逆順で巻き戻されていく
        let res = action_runner.undo_action();
        assert!(res.is_ok());
        assert_eq!(0, *undo_called_count1.borrow());
        assert_eq!(1, *undo_called_count2.borrow());

        let res = action_runner.undo_action();
        assert!(res.is_ok());
        assert_eq!(1, *undo_called_count1.borrow());
        assert_eq!(1, *undo_called_count2.borrow());

        // 全ての action を undo した後でも undo を呼ぶこと自体は許容される
        let res = action_runner.undo_action();
        assert!(res.is_ok());
        assert_eq!(1, *undo_called_count1.borrow());
        assert_eq!(1, *undo_called_count2.borrow());
    }
}
