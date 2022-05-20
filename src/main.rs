use proc_macro_derive_describe::Describe;
use proc_macro_expr_binary::{expr_binary_dbg_working, expr_binary_swap_and_subtract};
use proc_macro_verbatim::verbatim;

#[allow(unused)]
#[derive(Describe)]
struct MyStruct {
    my_string: String,
    my_enum: MyEnum,
    my_number: f64,
}

#[derive(Describe)]
struct MyTupleStruct(u32, String, i8);

#[allow(unused)]
#[derive(Describe)]
enum MyEnum {
    VariantA,
    VariantB,
}

#[allow(unused)]
#[derive(Describe)]
union MyUnion {
    unsigned: u32,
    signed: i32,
}

fn main() {
    MyStruct::describe();
    MyTupleStruct::describe();
    MyEnum::describe();
    MyUnion::describe();

    verbatim!(println!("hello, {}", "world"));

    expr_binary_dbg_working!(a + b);

    // If the `expr_binary_deb_working!(a b)` is uncommented
    // a nice compiler error is generated:
    //   error: expected binary operation
    //     --> src/main.rs:39:30
    //      |
    //   39 |     expr_binary_dbg_working!(a b);
    //      |                              ^
    //
    //   error: could not compile `expr-proc-macros` due to previous error
    //expr_binary_dbg_working!(a b);

    let a = 2;
    let b = 1;
    let res = expr_binary_swap_and_subtract!(a + b);
    println!("expr_binary_to_swap_and_substract: res={}", res);
    assert_eq!(b - a, res);
}
