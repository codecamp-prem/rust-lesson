fn main(){
    let a:Option<i32> = None;
    dbg!(a); // [src\bin\option_combinator_demo.rs:3] a = None

    let a_is_some = a.is_some();
    dbg!(a_is_some); // [src\bin\option_combinator_demo.rs:6] a_is_some = false

    let a_is_none = a.is_none();
    dbg!(a_is_none); // [src\bin\option_combinator_demo.rs:9] a_is_none = true

    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped); // [src\bin\option_combinator_demo.rs:12] a_mapped = None

    let a_filtered = a.filter(|num| num==&1); // filter uses borrowed
    dgb!(a_filtered); // [src\bin\option_combinator_demo.rs:15] a_filtered = None

    let a_or_else = a.or_else(|| Some(5));
    dgb!(a_or_else); // [src\bin\option_combinator_demo.rs:18] a_or_else = Some(5)

    let unwrapped = a.unwrap_or_else(|| 0);
    dgb!(unwrapped); // [src\bin\option_combinator_demo.rs:21] unwrapped = 0
}