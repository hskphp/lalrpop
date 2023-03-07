mod ast;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub calc);
#[test]
fn calc() {
    let expr = calc::exprParser::new().parse("22 * 44 + 66").unwrap();
    println!("{:?}", expr.clone());
    assert_eq!(&format!("{:?}", expr),
        "Add(Mul(Int(22), Int(44)), Int(66))");
}
fn main() {
    println!("Hello, world!");
}
