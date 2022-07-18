# Hierarchical State Machine (HSM) proc macro

Define a `proc_macro` to make it easier to create HSM's.

# Examples

The simplest HSM is just Finite State Machine (FSM) with a single
state and no hierarchical structure.

```
# use proc_macro_hsm1::{hsm1, hsm1_state, handled};

hsm1!(
    #[derive(Debug)]
    struct MyFsm {
        count: u64,
    }

    #[hsm1_state]
    fn initial(&mut self) -> StateResult {
        // Mutate the state
        self.count += 1;

        // Return the desired StateResult
        handled!()
    }
);

fn main() {
    let mut fsm = MyFsm::new();

    fsm.dispatch();
    println!("fsm: fsm.count={}", fsm.count);
    assert_eq!(fsm.count, 1);
}
```

Here is the simplest HSM with two states
```
# use proc_macro_hsm1::{hsm1, hsm1_state, handled, not_handled};

hsm1!(
    #[derive(Debug)]
    struct MyHsm {
        base_count: u64,
        initial_count: u64,
    }

    #[hsm1_state]
    fn base(&mut self) -> StateResult {
        // Mutate the state
        self.base_count += 1;

        // Return the desired StateResult
        handled!()
    }

    #[hsm1_state(base)]
    fn initial(&mut self) -> StateResult {
        // Mutate the state
        self.initial_count += 1;

        // Let the parent state handle all invocations
        not_handled!()
    }
);

fn main() {
    let mut hsm = MyHsm::new();

    hsm.dispatch();
    println!("hsm: hsm base_count={} intial_count={}", hsm.base_count, hsm.initial_count);
    assert_eq!(hsm.base_count, 1);
    assert_eq!(hsm.initial_count, 1);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

