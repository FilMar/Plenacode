use std::task::Context;



fn main() {
    println!("dio saltinbanco");
}


struct Context;

trait PlenaCode {
    fn exec(&self, ctx: Context) -> Context;
}

enum TestCode {
    Caldo,
    Incremento,
    Decremento,
}

impl PlenaCode for TestCode {
    fn exec(&self, ctx: Context) -> Context {
        match self {
            TestCode::Caldo => Context,
            TestCode::Decremento =>  Context,
            TestCode::Incremento =>  Context
        }
    }
}
