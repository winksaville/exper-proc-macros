use proc_macro_fsm1::{fsm1, fsm1_state};

fsm1!(
    struct MyFsm {
        a_i32: i32,
    }

    fn non_state_fn(& mut self) {
        self.a_i32 += 1;
        println!("MyFSM: non_state_fns_handle self.data={}", self.a_i32);
    }

    fn initial_entry(&mut self) {
        println!("MyFSM: initial_entry self.a_i32={}", self.a_i32);
    }

    #[fsm1_state]
    fn initial(& mut self) -> StateResult {
        self.non_state_fn();
        println!("MyFSM: initial self.a_i32={}", self.a_i32);

        StateResult::TransitionTo(1) //Self::do_work)
    }

    #[fsm1_state]
    fn do_work(& mut self) -> StateResult {
        self.a_i32 += 1;
        println!("MyFSM: do_work self.a_i32={}", self.a_i32);

        StateResult::TransitionTo(2) //Self::done)
    }

    #[fsm1_state]
    fn done(& mut self) -> StateResult {
        self.a_i32 += 1;
        println!("MyFSM: done self.a_i32={}", self.a_i32);

        StateResult::Handled
    }
);


fn main() {
    // Verify new without type works
    let mut my_new_fsm = MyFsm::new();
    println!("main: my_new_fsm={}", my_new_fsm.sm.state_fns[0].process as u64);
    my_new_fsm.a_i32 = 321;
    assert_eq!(my_new_fsm.a_i32, 321);

    // Verify new with type works
    let mut my_new_fsm: MyFsm = MyFsm::new();
    my_new_fsm.a_i32 = 456;
    assert_eq!(my_new_fsm.a_i32, 456);

    // Verify default without type works
    let mut _my_new_fsm = MyFsm::default();
    my_new_fsm.a_i32 = 213;
    assert_eq!(my_new_fsm.a_i32, 213);

    // Verify default with type works
    let mut my_fsm: MyFsm = Default::default();
    assert_eq!(my_fsm.sm.current_state_fns_handle as usize, 0); //MyFsm::initial as usize);
    assert_eq!(my_fsm.sm.previous_state_fns_handle as usize, 0); //MyFsm::initial as usize);
    assert!(my_fsm.sm.current_state_changed);

    my_fsm.a_i32 = 123;
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);

    // Invoke initial
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={} csc={}", my_fsm.a_i32, my_fsm.sm.current_state_changed);
    assert_eq!(my_fsm.sm.current_state_fns_handle as usize, 1); //MyFsm::do_work as usize);
    assert_eq!(my_fsm.sm.previous_state_fns_handle as usize, 0); //MyFsm::initial as usize);
    assert!(my_fsm.sm.current_state_changed);

    // Invoke do_work
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    assert_eq!(my_fsm.sm.current_state_fns_handle as usize, 2); //MyFsm::done as usize);
    assert_eq!(my_fsm.sm.previous_state_fns_handle as usize, 1); //MyFsm::do_work as usize);
    assert!(my_fsm.sm.current_state_changed);

    // Invoke done
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    assert_eq!(my_fsm.sm.current_state_fns_handle as usize, 2); //MyFsm::done as usize);
    assert_eq!(my_fsm.sm.previous_state_fns_handle as usize,  1); //MyFsm::do_work as usize);
    assert!(!my_fsm.sm.current_state_changed);

    // Invoke done again
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    assert_eq!(my_fsm.sm.current_state_fns_handle as usize, 2); //MyFsm::done as usize);
    assert_eq!(my_fsm.sm.previous_state_fns_handle as usize, 1); //MyFsm::do_work as usize);
    assert!(!my_fsm.sm.current_state_changed);
}


#[cfg(test)]
mod tests {
    use proc_macro_fsm1::{fsm1, fsm1_state};

    #[test]
    fn test_initialization_via_default() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let fsm: Test = Default::default();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //Test::initial as usize);
        assert!(fsm.sm.current_state_changed);
    }

    #[test]
    fn test_initialization_via_new() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //Test::initial as usize);
        assert!(fsm.sm.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //Test::initial as usize);
        assert!(!fsm.sm.current_state_changed);
    }

    #[test]
    fn test_transition_to() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> StateResult {
                StateResult::TransitionTo(1usize) //Test::done)
            }

            #[fsm1_state]
            fn done(& mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let mut fsm = Test::new();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 0); //Test::initial as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //Test::initial as usize);
        assert!(fsm.sm.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 1); //Test::done as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //Test::initial as usize);
        assert!(fsm.sm.current_state_changed);
    }

    #[test]
    fn test_dispatch() {
        fsm1!(
            struct TestDispatch {}

            #[fsm1_state]
            fn initial(& mut self) -> StateResult {
                StateResult::TransitionTo(1usize) //TestDispatch::done)
            }

            #[fsm1_state]
            fn done(& mut self) -> StateResult {
                StateResult::Handled
            }
        );

        let mut fsm = TestDispatch::new();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 0); //TestDispatch::initial as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //TestDispatch::initial as usize);
        assert!(fsm.sm.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 1); //TestDispatch::done as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //TestDispatch::initial as usize);
        assert!(fsm.sm.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 1); //TestDispatch::done as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //TestDispatch::initial as usize);
        assert!(!fsm.sm.current_state_changed);

        fsm.dispatch();
        assert_eq!(fsm.sm.current_state_fns_handle as usize, 1); //TestDispatch::done as usize);
        assert_eq!(fsm.sm.previous_state_fns_handle as usize, 0); //TestDispatch::initial as usize);
        assert!(!fsm.sm.current_state_changed);
    }
}
