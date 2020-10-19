mod test_with_v8;
use crate::test_with_v8::test_js;
//TODO: validation
#[derive(Debug, Clone)]
enum Branch<'br> {
    Add(Box<Branch<'br>>, Box<Branch<'br>>),
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
    BlockFunction {
        body: Vec<Branch<'br>>,
        params: Vec<&'br str>,
    },
    Return(Box<Branch<'br>>),
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
            Branch::Add(lhs, rhs) => return format!("{} + {}", lhs.to_js(), rhs.to_js()),
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
        }
    }
}
fn walk_branch<'a>(br: &'a Branch, f: fn()) -> Branch<'a> {
    println!("walk branch called {:?}", br);
    return match br {
        Branch::Add(l, r) => {
            let lh = walk_branch(&*l, f);
            let rh = walk_branch(&*r, f);
            Branch::Add(Box::new(lh), Box::new(rh))
        }
        Branch::Function { name, params, body } => {
            let v = body
                .iter()
                .map(|b| walk_branch(b, f))
                .collect::<Vec<Branch>>();
            Branch::Function {
                name: name,
                params: params.to_vec(), // TODO: Remove to_vec(), its ugly
                body: v,
            }
            // body.push(Branch::Block(vec![]));
        }
        Branch::Const(c) => Branch::Const(*c),
        Branch::Variable(v) => Branch::Variable(v),
        Branch::Assignment(l, r) => {
            let a = walk_branch(r, f);
            Branch::Assignment(l, Box::new(a))
        }
        Branch::LambdaFunction { params, body } => {
            let v = body
                .iter()
                .map(|b| walk_branch(b, f))
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
                .map(|b| walk_branch(b, f))
                .collect::<Vec<Branch>>();
            Branch::Block(v)
        }
        Branch::BlockFunction { body, params } => {
            let v = body
                .iter()
                .map(|b| walk_branch(b, f))
                .collect::<Vec<Branch>>();
            Branch::BlockFunction {
                body: v,
                params: params.to_vec(),
            }
        }
        Branch::Return(b) => {
            let r = walk_branch(b, f);
            Branch::Return(Box::new(r))
        }
    };
}
#[derive(Debug)]
struct Tree<'tr> {
    branches: Vec<Branch<'tr>>,
    flatten_lambdas: bool,
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
    pub fn lambda(&mut self, body: Vec<Branch<'a>>, params: Vec<&'a str>) -> Branch<'a> {
        if self.flatten_lambdas {
            let b = Branch::BlockFunction {
                body: body,
                params: params,
            };
            return b;
        } else {
            let b = Branch::LambdaFunction {
                body: body,
                params: params,
            };
            return b;
        }
    }
}

fn main() {
    let mut branches = vec![];
    // {
    //     let adder = Branch::Function {
    //         name: "add",
    //         body: vec![Branch::Return(Box::new(Branch::Add(
    //             Box::new(Branch::Variable("lhs")),
    //             Box::new(Branch::Variable("rhs")),
    //         )))],
    //         params: vec!["lhs", "rhs"],
    //     };
    //     branches.push(adder);
    // }
    // {
    //     let adder2 = Branch::Function {
    //         name: "add2",
    //         body: vec![Branch::Return(Box::new(Branch::Add(
    //             Box::new(Branch::Variable("lhs")),
    //             Box::new(Branch::Variable("rhs")),
    //         )))],
    //         params: vec!["lhs", "rhs"],
    //     };
    //     let x = Branch::Assignment("x", Box::new(adder2));

    //     branches.push(x)
    // }
    // {
    //     let lambda_body = vec![Branch::Return(Box::new(Branch::Add(
    //         Box::new(Branch::Variable("a")),
    //         Box::new(Branch::Add(
    //             Box::new(Branch::Variable("b")),
    //             Box::new(Branch::Variable("c")),
    //         )),
    //     )))];

    //     let lambda = tree.lambda(lambda_body, vec!["a", "b", "c"]);

    //     let lambda_owner = Branch::Assignment("lambda_owner", Box::new(lambda));
    //     branches.push(lambda_owner);
    // }
    // {
    //     let mut body = vec![
    //         Branch::Assignment("temp", Box::new(Branch::Const(42))),
    //         Branch::Return(Box::new(Branch::Variable("temp"))),
    //     ];
    //     let t = Branch::Assignment("temp", Box::new(Branch::Block(body)));
    //     // branches.push(t.clone());
    //     // branches.push(t.clone());
    //     // branches.push(t.clone());
    //     // branches.push(t.clone());
    //     // branches.push(t.clone())
    // }
    // tree.branches.append(&mut branches);

    branches.push(Branch::LambdaFunction {
        params: vec!["a", "b"],
        body: vec![Branch::Add(
            Box::new(Branch::Variable("a")),
            Box::new(Branch::Variable("b")),
        )],
    });
    let no_lambdas = true;

    let b = if no_lambdas {
        branches
            .iter()
            .map(|b| walk_branch(b, || {}))
            .collect::<Vec<Branch>>()
    } else {
        branches
    };

    let mut tree = Tree {
        branches: b,
        flatten_lambdas: false,
    };
    // for branch in &tree.branches {
    //     let walked = walk_branch(branch, || {});
    //     println!("walked = {:?}", walked);
    // }

    println!("//rendered tree = \n{}", tree.to_js());

    test_js(tree.to_js());
}
