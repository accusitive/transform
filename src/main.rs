#[derive(Debug)]
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
}
impl<'a> Branch<'a> {
    pub fn to_js(&self) -> String {
        match self {
            Branch::Add(lhs, rhs) => return format!("{} + {}", lhs.to_js(), rhs.to_js()),
            Branch::Const(c) => return c.to_string(),
            Branch::Function { name, params, body } => {
                let js_body = body
                    .iter()
                    .map(|b| b.to_js())
                    .collect::<Vec<String>>()
                    .join("\n\tasd");
                let js_args = params.join(", ");
                format!(
                    "function {name}({js_args}){{\n\t{js_body}\n}}",
                    name = name,
                    js_args = js_args,
                    js_body = js_body
                )
            }
            Branch::Variable(v) => format!("{}", v),
            Branch::Assignment(left, right) => format!("var {} = {}", left, right.to_js()),
            Branch::LambdaFunction { params, body } => {
                let js_body = body
                    .iter()
                    .map(|b| b.to_js())
                    .collect::<Vec<String>>()
                    .join("\n\tasd");
                let js_args = params.join(", ");
                format!("({}) => {{{}}}", js_args, js_body)
            }
        }
    }
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
/*

function add(a, b) {
    return a + b
}

*/
fn main() {
    let mut branches = vec![];
    {
        let adder = Branch::Function {
            name: "add",
            body: vec![Branch::Add(
                Box::new(Branch::Variable("lhs")),
                Box::new(Branch::Variable("rhs")),
            )],
            params: vec!["lhs", "rhs"],
        };
        branches.push(adder);
    }
    {
        let adder2 = Branch::Function {
            name: "add2",
            body: vec![Branch::Add(
                Box::new(Branch::Variable("lhs")),
                Box::new(Branch::Variable("rhs")),
            )],
            params: vec!["lhs", "rhs"],
        };
        let x = Branch::Assignment("x", Box::new(adder2));

        branches.push(x)
    }
    {
        let lambda_body = vec![
            Branch::Add(Box::new(Branch::Variable("a")), Box::new(Branch::Add(Box::new(Branch::Variable("b")), Box::new(Branch::Variable("c")))))
        ];
        let lambda = Branch::LambdaFunction{
            body: lambda_body,
            params: vec!["a", "b", "c"]
        };
        let lambda_owner = Branch::Assignment("lambda_owner", Box::new(lambda));
        branches.push(lambda_owner);
    }
    let mut tree = Tree { branches: branches };
    println!("//rendered tree = \n{}", tree.to_js());
}
