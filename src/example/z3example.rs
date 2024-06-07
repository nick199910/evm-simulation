use z3::{ast, Config, Context, SatResult, Solver};

#[test]
pub fn test_z3() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let solver = Solver::new(&ctx);

    let x = ast::Int::new_const(&ctx, "x");
    let y = ast::Int::new_const(&ctx, "y");

    solver.assert(&x.gt(&ast::Int::from_i64(&ctx, 5)));
    solver.assert(&y.gt(&ast::Int::from_i64(&ctx, 10)));
    solver.assert(&ast::Int::add(&ctx, &[&x, &y]).gt(&ast::Int::from_i64(&ctx, 100)));
    match solver.check() {
        SatResult::Sat => {
            println!("{:?}", solver.get_model().unwrap());
        }
        SatResult::Unsat => {
            println!("{:?}", solver.get_model().unwrap());
        }
        SatResult::Unknown => println!("..."),
    }
}
