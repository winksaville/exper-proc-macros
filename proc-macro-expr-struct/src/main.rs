fn manual_implementation() {
    type ProcessFn = fn(&mut X);

    struct X {
        do_process: ProcessFn,
        data: u32,
    }

    impl X {
        fn new() -> Self {
            //init!()
            Self {
                do_process: Self::process,
                data: 0,
            }
        }

        fn dispatch(&mut self) {
            (self.do_process)(self)
        }

        fn process(&mut self) {
            self.data += 1;
        }
    }

    let mut x = X::new();
    assert_eq!(x.data, 0);
    println!("X: x.data={}", x.data);
    x.dispatch();
    assert_eq!(x.data, 1);
    println!("X: x.data={}", x.data);
}

fn main() {
    manual_implementation();
}

