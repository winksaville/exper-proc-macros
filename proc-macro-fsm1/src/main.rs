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

    // Works but my_fsm.sm is not visible to vs_code code completion
    assert_eq!(my_fsm.sm.current_state_changed, true);
    my_fsm.sm.current_state_changed = false;
    assert_eq!(my_fsm.sm.current_state_changed, false);

    println!("main: my_fsm={:?}", my_fsm);
    my_fsm.non_state_fn();
    let _ = my_fsm.initial();
    let _ = my_fsm.do_work();
    let _ = my_fsm.done();
    println!("main: my_fsm={:?}", my_fsm);

}