use proc_macro_fsm1::{fsm1, fsm1_state};


fsm1!(
    MyFsm {
        a_i32: i32,
        a_u32: u32,
    }
    fn non_state_fn(&self) {
        println!("MyFSM: non_state_fn self={:#?}", self);
    }
    #[fsm1_state]
    fn initial(&self) {
        println!("MyFSM: initial self={:#?}", self);
    }
    #[fsm1_state]
    fn do_work(&self) {
        println!("MyFSM: do_work self={:#?}", self);
    }
    #[fsm1_state]
    fn done(&self) {
        println!("MyFSM: done self={:#?}", self);
    }
);

fn main() {
    let my_fsm = MyFsm {
        a_i32: 0i32,
        a_u32: 1u32,
    };
    println!("main: my_fsm={:#?}", my_fsm);
    my_fsm.non_state_fn();
    my_fsm.initial();
    my_fsm.do_work();
    my_fsm.done();
}