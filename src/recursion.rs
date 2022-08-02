// https://recursion.wtf/posts/rust_schemes/
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    iter::repeat_with,
    mem::{replace, MaybeUninit},
};

#[derive(Debug, Clone)]
pub enum ExprBoxed {
    Add {
        a: Box<ExprBoxed>,
        b: Box<ExprBoxed>,
    },

    Sub {
        a: Box<ExprBoxed>,
        b: Box<ExprBoxed>,
    },

    Mul {
        a: Box<ExprBoxed>,
        b: Box<ExprBoxed>,
    },
    LiteralInt {
        literal: i64,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ExprLayer<A> {
    Add { a: A, b: A },
    Sub { a: A, b: A },
    Mul { a: A, b: A },
    LiteralInt { literal: i64 },
}

impl ExprBoxed {
    pub fn eval(&self) -> i64 {
        match &self {
            ExprBoxed::Add { a, b } => a.eval() + b.eval(),
            ExprBoxed::Sub { a, b } => a.eval() - b.eval(),
            ExprBoxed::Mul { a, b } => a.eval() * b.eval(),
            ExprBoxed::LiteralInt { literal } => *literal,
        }
    }
}

#[derive(Debug)]
struct ExprdIdx(usize);

impl ExprdIdx {
    fn head() -> Self {
        ExprdIdx(0)
    }
}

pub struct ExprTopo {
    elems: Vec<ExprLayer<ExprdIdx>>,
}

impl ExprTopo {
    fn eval(self) -> i64 {
        let mut results = repeat_with(|| MaybeUninit::<i64>::uninit())
            .take(self.elems.len())
            .collect::<Vec<_>>();

        fn get_result_unsafe(results: &mut Vec<MaybeUninit<i64>>, idx: ExprdIdx) -> i64 {
            unsafe {
                let maybe_uninit = replace(results.get_unchecked_mut(idx.0), MaybeUninit::uninit());
                maybe_uninit.assume_init()
            }
        }

        for (idx, node) in self.elems.into_iter().enumerate().rev() {
            let result = {
                // each node is only referenced once so just remove it, also we know it's there so unsafe is fine
                match node {
                    ExprLayer::Add { a, b } => {
                        let a = get_result_unsafe(&mut results, a);
                        let b = get_result_unsafe(&mut results, b);
                        a + b
                    }
                    ExprLayer::Sub { a, b } => {
                        let a = get_result_unsafe(&mut results, a);
                        let b = get_result_unsafe(&mut results, b);
                        a - b
                    }
                    ExprLayer::Mul { a, b } => {
                        let a = get_result_unsafe(&mut results, a);
                        let b = get_result_unsafe(&mut results, b);
                        a * b
                    }
                    ExprLayer::LiteralInt { literal } => literal,
                }
            };
            results[idx].write(result);
        }

        unsafe {
            let maybe_uninit =
                std::mem::replace(results.get_unchecked_mut(0), MaybeUninit::uninit());
            maybe_uninit.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name() {
        ExprBoxed::Mul {
            a: Box::new(ExprBoxed::LiteralInt { literal: 1 }),
            b: Box::new(ExprBoxed::Sub {
                a: Box::new(ExprBoxed::LiteralInt { literal: 2 }),
                b: Box::new(ExprBoxed::LiteralInt { literal: 3 }),
            }),
        }
    }
}
