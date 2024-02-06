use crate::utils;
#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
#[derive(Debug, PartialEq)]
pub struct Evaluate(pub i32);

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}



#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}
impl Number {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, number) = utils::extract_digits(s);
        (s, Self(number.parse().unwrap()))
    }
}
impl Op {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, op) = utils::extract_op(s);
        let op = match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => unreachable!(),
        };

        (s, op)
    }
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, op) = Op::new(s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, rhs) = Number::new(s);

        (s, Self { lhs, rhs, op })
        
    }
}

impl Evaluate{
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = Number::new(s);
         let (s, _) = utils::extract_whitespace(s);
        let (s, op) = Op::new(s);
        let (s, _) = utils::extract_whitespace(s);
        let (s, rhs) = Number::new(s);
        
        let result = match op {
            Op::Add => lhs.0 + rhs.0,
            Op::Sub => lhs.0 - rhs.0,
            Op::Mul => lhs.0 * rhs.0,
            Op::Div => lhs.0 / rhs.0,
        };
        (s, Self(result))
    }
}
//#[cfg(test)]
// mod tests {
//     use super::*;
//     fn parse_number() {
//         assert_eq!(Number::new("123"), ("", Number(123)));
//     }

//     #[test]
//     fn parse_add_op() {
//         assert_eq!(Op::new("+"), ("", Op::Add));
//     }

//     #[test]
//     fn parse_sub_op() {
//         assert_eq!(Op::new("-"), ("", Op::Sub));
//     }

//     #[test]
//     fn parse_mul_op() {
//         assert_eq!(Op::new("*"), ("", Op::Mul));
//     }

//     #[test]
//     fn parse_div_op() {
//         assert_eq!(Op::new("/"), ("", Op::Div));
//     }
//     #[test]
//     fn parse_one_plus_two() {
//         assert_eq!(
//             Expr::new("1+2"),
//             (
//                 "",
//                 Expr {
//                     lhs: Number(1),
//                     rhs: Number(2),
//                     op: Op::Add,
//                 },
//             ),
//         );
//     }
//     #[test]
//     fn parse_expr_with_whitespace() {
//         assert_eq!(
//             Expr::new("2 * 2"),
//             (
//                 "",
//                 Expr {
//                     lhs: Number(2),
//                     rhs: Number(2),
//                     op: Op::Mul,
//                 },
//             ),
//         );
//     }
//     #[test]
//     fn parse_operation(){
//         assert_eq!(Evaluate::new("1+2"), ("", Evaluate(3)));
//     }
// }
