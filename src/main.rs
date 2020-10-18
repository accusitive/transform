
#[derive(Debug)]
enum Branch {
    Add(Box<Branch>, Box<Branch>),
    Const(i32)
}
impl<'a> Branch {
    pub fn to_js(&self) -> String {
        match self {
            Branch::Add(lhs , rhs) => {
                return format!("{} + {}", lhs.to_js(), rhs.to_js())
            }
            Branch::Const(c) => {
                return c.to_string()
            }
        }
    }
}
#[derive(Debug)]
struct Tree {
    branches: Vec<Branch>,
}
impl<'a> Tree {
    pub fn to_js(&mut self) -> String{
        let compiled_branches = self.branches.iter().map(|b: &Branch|b.to_js()).collect::<Vec<String>>();

        compiled_branches.join(";\n")
    }
}
fn increment_branch(b: &mut Branch) -> Branch{
    let r = match b{

        Branch::Add(lhs, rhs) => {
            let l = Branch::Add(Box::new(increment_branch(lhs)), Box::new(Branch::Const(1)));
            let r = Branch::Add(Box::new(increment_branch(rhs)), Box::new(Branch::Const(1)));

            Branch::Add(Box::new(l),Box::new(r))
        }
        Branch::Const(i) => {
            //println!("returning a const + 1");
            Branch::Const(*i+1)
        }
    };
    r
}
fn increment_consts(t: &mut Tree) {
    for branch in &mut t.branches {
        let b = increment_branch(branch);
        *branch = b;
    }
}
fn add(lhs: i32, rhs: i32) -> Branch{
    Branch::Add(
        Box::new(Branch::Const(lhs)),
        Box::new(Branch::Const(rhs)),
    )
}
fn addb(lhs: Branch, rhs: Branch) -> Branch{
    Branch::Add(
        Box::new(lhs),
        Box::new(rhs),
    )
}
/*

function add(a, b) {
    return a + b
}

*/
fn main() {
    let mut tree = Tree {
        branches: {
            vec![
                add(20, 60),
            ]
        },
    };
     println!("//rendered tree = \nconst a = {}\nconsole.log(a)", tree.to_js());
}
