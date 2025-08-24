fn main() {
    let a = 1; let b = 2.5; let c = true; let d = 'x'; let e = "Hello";
    let sum = a + 10; let cmp = b > 1.0; let text = format!("{} {}", e, a);
    struct Foo { x: i32, y: i32 } let f = Foo { x: 5, y: 6 };
    enum Bar { A, B } let bval = Bar::A;
    let vec = vec![1, 2, 3]; let map = std::collections::HashMap::new();
    let closure = |x:i32| x*x; let sq = closure(4);
    match a { 0 => println!("zero"), _ => println!("other") }
    for i in 0..3 { println!("{}", i); }
    #[allow(dead_code)] fn hidden() {} hidden();
}
