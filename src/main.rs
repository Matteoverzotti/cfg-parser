#[derive(Debug)]
enum Symbol {
    Terminal(char),
    NonTerminal(&'static str),
}

#[derive(Debug)]
struct Production {
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

#[derive(Debug)]
struct CFG {
    nonterminals: Vec<Symbol>,
    terminals: Vec<Symbol>,
    start_symbol: Symbol,
    productions: Vec<Production>,
}

impl CFG {
    fn build() -> Self {
        let productions = vec![
            Production {
                lhs: Symbol::NonTerminal("S"),
                rhs: vec![Symbol::Terminal('a'), Symbol::NonTerminal("S"), Symbol::Terminal('b')]
            },
            Production {
                lhs: Symbol::NonTerminal("S"),
                rhs: vec![] // ε = empty production
            }
        ];
        CFG {
            nonterminals: vec![Symbol::NonTerminal("S")],
            terminals: vec![Symbol::Terminal('a'), Symbol::Terminal('b')],
            start_symbol: Symbol::NonTerminal("S"),
            productions,
        }
    }
}

fn main() {
    let cfg = CFG::build();
    println!("{:#?}", cfg);
}
