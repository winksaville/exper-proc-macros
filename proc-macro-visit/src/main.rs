use proc_macro_visit::visit1;

visit1!(
    fn yo() {
        println!("visit1 macro output");
    }
);

fn main() {
    println!("Hi");
    yo();

}

#[cfg(test)]
mod tests {
    //use proc_macro_fsm1::{fsm1, fsm1_state};

    #[test]
    fn test_x() {
    }
}
