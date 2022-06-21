use proc_macro_fsm1::{fsm1, fsm1_state};

fsm1!(
    struct MyFsm {
        a_i32: i32,
        a_u32: u32,
    }

    fn non_state_fn(& mut self) {
        self.a_i32 += 1;
        self.a_u32 -= 1;
        println!("MyFSM: non_state_fn self={:?}", self);
    }

    #[fsm1_state]
    fn initial(& mut self) -> bool {
        self.a_i32 += 1;
        self.a_u32 -= 1;
        self.transition_to(MyFsm::do_work);
        println!("MyFSM: initial self.a_i32={:?}", self);
        true
    }

    #[fsm1_state]
    fn do_work(& mut self) -> bool {
        self.a_i32 += 1;
        self.a_u32 -= 1;
        println!("MyFSM: do_work self={:?}", self);
        true
    }

    #[fsm1_state]
    fn done(& mut self) -> bool {
        self.a_i32 += 1;
        self.a_u32 -= 1;
        println!("MyFSM: done self={:?}", self);
        true
    }
);


fn main() {
    // Verify new without type works
    let mut my_new_fsm = MyFsm::new();
    my_new_fsm.a_i32 = 321;
    my_new_fsm.a_u32 = 321;
    assert_eq!(my_new_fsm.a_i32, 321);

    // Verify new with type works
    let mut my_new_fsm: MyFsm = MyFsm::new();
    my_new_fsm.a_i32 = 456;
    my_new_fsm.a_u32 = 456;
    assert_eq!(my_new_fsm.a_i32, 456);

    // Verify default without type works
    let mut my_new_fsm = MyFsm::default();
    my_new_fsm.a_i32 = 213;
    my_new_fsm.a_u32 = 213;

    // Verify default with type works
    let mut my_fsm: MyFsm = Default::default();
    my_fsm.a_i32 = 123;
    my_fsm.a_u32 = 123;

    assert!(my_fsm.sm.current_state == MyFsm::initial, "current_state != MyFsm::initial");
    assert!(my_fsm.sm.previous_state == MyFsm::initial, "previous_state != MyFsm::initial");
    assert_eq!(my_fsm.sm.current_state_changed, true);
    my_fsm.sm.current_state_changed = false;
    assert_eq!(my_fsm.sm.current_state_changed, false);

    println!("main: my_fsm={:?}", my_fsm);
    my_fsm.non_state_fn();
    assert!(my_fsm.sm.current_state == MyFsm::initial, "current_state != MyFsm::initial");
    assert!(my_fsm.sm.previous_state == MyFsm::initial, "previous_state != MyFsm::initial");
    assert!(my_fsm.sm.current_state_changed == false, "current_state_changed != false");
    _ = my_fsm.initial();
    assert!(my_fsm.sm.current_state == MyFsm::do_work, "current_state != MyFsm::do_work");
    assert!(my_fsm.sm.previous_state == MyFsm::initial, "previous_state != MyFsm::initial");
    assert!(my_fsm.sm.current_state_changed == true, "current_state_changed != true");
    _ = my_fsm.do_work();
    _ = my_fsm.done();
    println!("main: my_fsm={:?}", my_fsm);
}

#[cfg(test)]
mod tests {
    use proc_macro_fsm1::{fsm1, fsm1_state};

    #[test]
    fn test_initialization_via_default() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                true
            }
        );

        let fsm: Test = Default::default();
        assert!(fsm.sm.current_state == Test::initial, "current_state != Test1::initial");
        assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
    }

    #[test]
    fn test_initialization_via_new() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                true
            }
        );

        let fsm = Test::new();
        assert!(fsm.sm.current_state == Test::initial, "current_state != Test1::initial");
        assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
    }

    #[test]
    fn test_transition_to() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                self.transition_to(Test::done);
                true
            }

            #[fsm1_state]
            fn done(& mut self) -> bool {
                true
            }
        );

        let mut fsm = Test::new();
        assert!(fsm.sm.current_state == Test::initial, "current_state != Test1::initial");
        assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
        fsm.sm.current_state_changed = false;
        _ = fsm.initial();
        assert!(fsm.sm.current_state == Test::done, "current_state != Test1::done");
        assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
    }
}