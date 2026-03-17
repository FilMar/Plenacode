fn main() {
}


struct Context;

trait PlenaCode {
    fn update_context(&self, ctx: Context) -> Context;
}

enum SpellTest {
    Calore, 
    Energia,
    Aumentare,
    Diminuire,
    Spingere,
    Tirare,
    Vita,
    Morte,
    Vicino,
    Lontano,
    Direzione,
    Area,
    Bersaglio,
    Ora,
    Dopo
}

impl PlenaCode for SpellTest {
    fn update_context(&self, ctx: Context) -> Context {
        
    }
}
