use proc_macro_fsm1::{fsm1, fsm1_state};


fsm1!(
    MyFsm {
        a_i32: i32,
    }

    fn non_state_fn(& mut self) {
        self.a_i32 += 1;
        //println!("MyFSM: non_state_fn self={:#?}", self); // Debug not implemented
        println!("MyFSM: non_state_fn self.a_i32={:#?}", self.a_i32);
    }

    #[fsm1_state]
    fn initial(& mut self) -> bool {
        self.a_i32 += 1;
        //println!("MyFSM: initial self={:#?}", self); // Debug not implemented
        println!("MyFSM: initial self.a_i32={:#?}", self.a_i32);
        true
    }

    #[fsm1_state]
    fn do_work(& mut self) -> bool {
        self.a_i32 += 1;
        //println!("MyFSM: do_work self={:#?}", self); // Debug not implemented
        println!("MyFSM: do_work self.a_i32={:#?}", self.a_i32);
        true
    }

    #[fsm1_state]
    fn done(& mut self) -> bool {
        self.a_i32 += 1;
        //println!("MyFSM: done self={:#?}", self.a_i32); // Debug not implemented
        println!("MyFSM: done self.a_i32={:#?}", self.a_i32);
        true
    }
);

fn main() {
    // Works but my_new_fsm.a_i32 is not visible to vs_code code completion
    let mut my_new_fsm = MyFsm::new();
    my_new_fsm.a_i32 = 321;
    assert_eq!(my_new_fsm.a_i32, 321);

    let mut my_new_fsm: MyFsm = MyFsm::new();
    my_new_fsm.a_i32 = 456;
    assert_eq!(my_new_fsm.a_i32, 456);

    // Works but my_new_fsm.a_i32 is not visible to vs_code code completion
    let mut my_new_fsm = MyFsm::default();
    my_new_fsm.a_i32 = 213;
    assert_eq!(my_new_fsm.a_i32, 213);

    let mut my_fsm: MyFsm = Default::default();
    my_fsm.a_i32 = 123;

    // Works but my_fsm.sm is not visible to vs_code code completion
    assert_eq!(my_fsm.sm.current_state_changed, true);
    my_fsm.sm.current_state_changed = false;
    assert_eq!(my_fsm.sm.current_state_changed, false);

    //println!("main: my_fsm={:#?}", my_fsm);
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    my_fsm.non_state_fn();
    let _ = my_fsm.initial();
    let _ = my_fsm.do_work();
    let _ = my_fsm.done();
    println!("main: my_fsm.sm={:?}", my_fsm.sm);
    println!("main: my_fsm={:?}", my_fsm);

}