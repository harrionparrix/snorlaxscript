use crate::expr::Expr;
use crate::utils;
use crate::env::Env;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = utils::tag("make", s);
        let (s, _) = utils::extract_whitespace(s);

        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespace(s);

        let s = if s.starts_with('=') {
            &s[1..]
        } else {
            panic!("expected equals sign")
        };
        let (s, _) = utils::extract_whitespace(s);

        let (s, val) = Expr::new(s);

        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div,
                    },
                },
            )),
        );
    }
    

}