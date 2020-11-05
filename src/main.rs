mod test_with_v8;
use crate::test_with_v8::test_js;
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
    Lt,
    Gt,
    Gte,
    Lte,
    Ne,
    Eq,
    Set,
}
impl Op {
    fn to_js(self) -> String {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Div => "/",
            Op::Mul => "*",
            Op::Lt => "<",
            Op::Gt => ">",
            Op::Gte => ">=",
            Op::Lte => "<=",
            Op::Ne => "!=",
            Op::Eq => "==",
            Op::Set => "=",
        }
        .to_string()
    }
}
//TODO: validation
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Branch<'br> {
    BinaryOp(Op, Box<Branch<'br>>, Box<Branch<'br>>),
    // Add(Box<Branch<'br>>, Box<Branch<'br>>),
    // ShiftRight(Box<Branch<'br>>, Box<Branch<'br>>),
    // Divide(Box<Branch<'br>>, Box<Branch<'br>>),
    Function {
        name: &'br str,
        params: Vec<&'br str>,
        body: Vec<Branch<'br>>,
    },
    Const(i32),
    Variable(&'br str), // Name
    Assignment(&'br str, Box<Branch<'br>>),
    LambdaFunction {
        params: Vec<&'br str>,
        body: Vec<Branch<'br>>,
    },
    Block(Vec<Branch<'br>>),
    ExpressionBlock(Vec<Branch<'br>>),

    BlockFunction {
        body: Vec<Branch<'br>>,
        params: Vec<&'br str>,
    },
    Return(Box<Branch<'br>>),
    CForLoop {
        init: Box<Branch<'br>>,
        check: Box<Branch<'br>>,
        action: Box<Branch<'br>>,
        inner: Box<Branch<'br>>,
    }, // for(init; check; action){inner},
    ConsoleLog(Box<Branch<'br>>),
}
macro_rules! render_body {
    ($body: ident) => {
        "\t".to_owned()
            + &$body
                .iter()
                .map(|b| b.to_js())
                .collect::<Vec<String>>()
                .join(";\n\t")
            + "\n"
    };
}
impl<'a> Branch<'a> {
    pub fn to_js(&self) -> String {
        match self {
            // Branch::Add(lhs, rhs) => return format!("{} + {}", lhs.to_js(), rhs.to_js()),
            Branch::Const(c) => return c.to_string(),
            Branch::Function { name, params, body } => {
                let js_body = render_body!(body);
                let js_args = params.join(", ");
                format!(
                    "function {name}({js_args}){{\n{js_body}\n}}",
                    name = name,
                    js_args = js_args,
                    js_body = js_body
                )
            }
            Branch::Variable(v) => format!("{}", v),
            Branch::Assignment(left, right) => format!("var {} = {}", left, right.to_js()),
            Branch::LambdaFunction { params, body } => {
                let js_body = render_body!(body);

                let js_args = params.join(", ");
                format!("({}) => {{\n{}\n}}", js_args, js_body)
            }
            Branch::Block(body) => {
                let js_body = render_body!(body);

                format!("(function (){{\n{}\n}})()", js_body)
            }
            Branch::BlockFunction { params, body } => {
                let js_body = render_body!(body);

                let js_args = params.join(", ");

                format!("(function ({}){{\n{}}})", js_args, js_body)
            }
            Branch::Return(v) => format!("return {}", v.to_js()),
            Branch::CForLoop {
                init,
                check,
                action,
                inner,
            } => format!(
                "for({}; {}; {}){{ {} }}",
                init.to_js(),
                check.to_js(),
                action.to_js(),
                inner.to_js()
            ),
            Branch::BinaryOp(op, lhs, rhs) => {
                format!("{} {} {}", lhs.to_js(), op.to_js(), rhs.to_js())
            }
            Branch::ExpressionBlock(body) => {
                let js_body = render_body!(body);

                format!("{}", js_body)
            }
            Branch::ConsoleLog(c) => format!("console.log({})", c.to_js()),
        }
    }
}
fn flatten_lambda<'a>(br: &'a Branch) -> Branch<'a> {
    return match br {
        Branch::BinaryOp(op, l, r) => {
            let lh = flatten_lambda(&*l);
            let rh = flatten_lambda(&*r);
            Branch::BinaryOp(op.to_owned(), Box::new(lh), Box::new(rh))
        }
        Branch::Function { name, params, body } => {
            let v = body
                .iter()
                .map(|b| flatten_lambda(b))
                .collect::<Vec<Branch>>();
            Branch::Function {
                name: name,
                params: params.to_vec(),
                body: v,
            }
        }
        Branch::Const(c) => Branch::Const(*c),
        Branch::Variable(v) => Branch::Variable(v),
        Branch::Assignment(l, r) => {
            let a = flatten_lambda(r);
            Branch::Assignment(l, Box::new(a))
        }
        Branch::LambdaFunction { params, body } => {
            let v = body
                .iter()
                .map(|b| flatten_lambda(b))
                .collect::<Vec<Branch>>();
            let lambda_to_bool = true;
            if lambda_to_bool {
                Branch::BlockFunction {
                    body: v,
                    params: params.to_vec(),
                }
            } else {
                Branch::LambdaFunction {
                    body: v,
                    params: params.to_vec(),
                }
            }
        }
        Branch::Block(body) => {
            let v = body
                .iter()
                .map(|b| flatten_lambda(b))
                .collect::<Vec<Branch>>();
            Branch::Block(v)
        }
        Branch::BlockFunction { body, params } => {
            let v = body
                .iter()
                .map(|b| flatten_lambda(b))
                .collect::<Vec<Branch>>();
            Branch::BlockFunction {
                body: v,
                params: params.to_vec(),
            }
        }
        Branch::Return(b) => {
            let r = flatten_lambda(b);
            Branch::Return(Box::new(r))
        }

        Branch::CForLoop {
            init,
            check,
            action,
            inner,
        } => Branch::CForLoop {
            init: Box::new(flatten_lambda(&*init)),
            check: Box::new(flatten_lambda(&*check)),
            action: Box::new(flatten_lambda(&*action)),
            inner: Box::new(flatten_lambda(&*inner)),
        },
        Branch::ExpressionBlock(body) => {
            let v = body
                .iter()
                .map(|b| flatten_lambda(b))
                .collect::<Vec<Branch>>();
            Branch::ExpressionBlock(v)
        }
        Branch::ConsoleLog(c) => Branch::ConsoleLog(Box::new(flatten_lambda(c))),
    };
}

fn unroll_loops<'a>(br: &'a Branch) -> Branch<'a> {
    return match br {
        Branch::BinaryOp(op, l, r) => {
            let lh = flatten_lambda(&*l);
            let rh = flatten_lambda(&*r);
            Branch::BinaryOp(op.to_owned(), Box::new(lh), Box::new(rh))
        }
        Branch::Function { name, params, body } => {
            let v = body
                .iter()
                .map(|b| unroll_loops(b))
                .collect::<Vec<Branch>>();
            Branch::Function {
                name: name,
                params: params.to_vec(),
                body: v,
            }
        }
        Branch::Const(c) => Branch::Const(*c),
        Branch::Variable(v) => Branch::Variable(v),
        Branch::Assignment(l, r) => {
            let a = unroll_loops(r);
            Branch::Assignment(l, Box::new(a))
        }
        Branch::LambdaFunction { params, body } => {
            let v = body
                .iter()
                .map(|b| unroll_loops(b))
                .collect::<Vec<Branch>>();
            let lambda_to_bool = true;
            if lambda_to_bool {
                Branch::BlockFunction {
                    body: v,
                    params: params.to_vec(),
                }
            } else {
                Branch::LambdaFunction {
                    body: v,
                    params: params.to_vec(),
                }
            }
        }
        Branch::Block(body) => {
            let v = body
                .iter()
                .map(|b| unroll_loops(b))
                .collect::<Vec<Branch>>();
            Branch::Block(v)
        }
        Branch::BlockFunction { body, params } => {
            let v = body
                .iter()
                .map(|b| unroll_loops(b))
                .collect::<Vec<Branch>>();
            Branch::BlockFunction {
                body: v,
                params: params.to_vec(),
            }
        }
        Branch::Return(b) => {
            let r = unroll_loops(b);
            Branch::Return(Box::new(r))
        }

        Branch::CForLoop {
            init,
            check,
            action,
            inner,
        } => Branch::CForLoop {
            init: Box::new(unroll_loops(&*init)),
            check: Box::new(unroll_loops(&*check)),
            action: Box::new(unroll_loops(&*action)),
            inner: Box::new(unroll_loops(&*inner)),
        },
        Branch::ExpressionBlock(body) => {
            let v = body
                .iter()
                .map(|b| flatten_lambda(b))
                .collect::<Vec<Branch>>();
            Branch::ExpressionBlock(v)
        }
        Branch::ConsoleLog(c) => Branch::ConsoleLog(Box::new(flatten_lambda(c))),
    };
}

#[derive(Debug)]
struct Tree<'tr> {
    branches: Vec<Branch<'tr>>,
}
impl<'a> Tree<'a> {
    pub fn to_js(&mut self) -> String {
        let compiled_branches = self
            .branches
            .iter()
            .map(|b: &Branch| b.to_js())
            .collect::<Vec<String>>();

        compiled_branches.join(";\n")
    }
}

fn main() {
    let mut branches = vec![];
    let increment_i = {
        let i_ref = Branch::Variable("i");
        let i_ref_plus_one = Branch::BinaryOp(Op::Add, Box::new(i_ref), Box::new(Branch::Const(1)));
        let i_ref = Branch::Variable("i");
        Branch::BinaryOp(Op::Set, Box::new(i_ref), Box::new(i_ref_plus_one))
    };

    let blk = Branch::Block(vec![
 
        Branch::CForLoop {
            init: Box::new(Branch::ExpressionBlock(vec![Branch::Assignment(
                "i",
                Box::new(Branch::Const(0)),
            )])),
            check: Box::new(Branch::ExpressionBlock(vec![Branch::BinaryOp(
                Op::Lte,
                Box::new(Branch::Variable("i")),
                Box::new(Branch::Const(100)),
            )])),
            action: Box::new(Branch::Block(vec![increment_i])),
            inner: Box::new(Branch::ConsoleLog(Box::new(Branch::Variable("i")))),
        },
    ]);
    branches.push(blk);

    let b = branches
        .iter()
        .map(|b| flatten_lambda(b))
        .collect::<Vec<Branch>>();
    let b = b.iter().map(|b| unroll_loops(b)).collect::<Vec<Branch>>();

    let mut tree = Tree { branches: b };

    println!("//rendered tree = \n{}", tree.to_js());

    test_js(tree.to_js());
}

#[test]
fn test_assignment() {
    let mut tree = Tree {
        branches: vec![Branch::Assignment("left", Box::new(Branch::Const(6)))],
    };
    assert_eq!(tree.to_js(), "var left = 6")
}

#[test]
fn test_lambda() {
    let mut tree = Tree {
        branches: vec![Branch::LambdaFunction {
            body: vec![],
            params: vec![],
        }],
    };
    assert_eq!(tree.to_js(), "() => {\n\t\n\n}")
}

#[test]
fn test_lambda_flatten_pass() {
    let lam = Branch::LambdaFunction {
        body: vec![],
        params: vec![],
    };
    let no_lambda = flatten_lambda(&lam);
    let mut tree = Tree {
        branches: vec![no_lambda],
    };
    // assert_eq!()
    assert_eq!(tree.to_js(), "(function (){\n\t\n})")
}
#[test]
fn test_block() {
    let blk = Branch::Block(vec![Branch::Return(Box::new(Branch::Add(
        Box::new(Branch::Const(22)),
        Box::new(Branch::Const(654)),
    )))]);

    let mut tree = Tree {
        branches: vec![blk],
    };
    assert_eq!(tree.to_js(), "(function (){\n\treturn 22 + 654\n\n})()");
}
