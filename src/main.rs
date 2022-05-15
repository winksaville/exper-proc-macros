use derive_macro::Describe;
use verbatium_proc_macro::verbatium;

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

    verbatium!(println!("hello, {}", "world"));
}
