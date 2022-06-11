use proc_macro_fsm1::fsm1;

fn main() {
    fsm1!(MyFsm {
              a_i32: i32,
              a_u32: u32,
        }
    );
}
