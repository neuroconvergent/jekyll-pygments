//! Doc comment
fn main() {
    // Keywords, literals, types
    let num: i32 = 42;
    let flt: f64 = 3.14;
    let boolean: bool = true;
    let s = "Hello";
    let c = 'c';

    // Operators, punctuation
    let sum = num + 1;
    let cmp = num > 0;

    // Functions, structs, enums, constants
    struct Foo { val: i32 }
    enum Bar { A, B }
    fn baz() {}
    const MAX: i32 = 100;

    // Attributes, macros, regex
    #[derive(Debug)] struct AttrTest;
    macro_rules! my_macro { () => {} }
    let regex = r"\d+";
}
