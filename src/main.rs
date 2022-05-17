use derive_macro::Describe;
use expr_binary::{expr_binary_dbg_not_working, expr_binary_swap_and_subtract};
use verbatim_proc_macro::verbatim;

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

    expr_binary_dbg_not_working!(a + b); // Does nothing, shows bug if dbg! statement enabled

    let a = 2;
    let b = 1;
    let res = expr_binary_swap_and_subtract!(a + b);
    println!("expr_binary_to_swap_and_substract: res={}", res);
    assert_eq!(b - a, res);
}
