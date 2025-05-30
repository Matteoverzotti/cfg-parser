#[derive(Debug, PartialEq, Eq)]
enum Symbol {
    Terminal(char),
    NonTerminal(char),
}

#[derive(Debug, PartialEq, Eq)]
struct Production {
    lhs: Symbol,
    rhs: Vec<Symbol>
}

#[derive(Debug)]
pub struct BonusCFG {
    _nonterminals: Vec<Symbol>,
    _terminals: Vec<Symbol>,
    _start_symbol: Symbol,
    _productions: Vec<Production>,
}

// S -> aSbSc | ε
impl BonusCFG {
    pub fn build() -> Self {
        let productions = vec![
            Production {
                lhs: Symbol::NonTerminal('S'),
                rhs: vec![Symbol::Terminal('a'), Symbol::NonTerminal('S'),
                        Symbol::Terminal('b'), Symbol::NonTerminal('S'),
                        Symbol::Terminal('c')]
            },
            Production {
                lhs: Symbol::NonTerminal('S'),
                rhs: vec![] // ε = empty production
            }
        ];
        BonusCFG { 
            _nonterminals: vec![Symbol::NonTerminal('S')], 
            _terminals: vec![Symbol::Terminal('a'), Symbol::Terminal('b'), Symbol::Terminal('c')], 
            _start_symbol: Symbol::NonTerminal('S'), 
            _productions: productions 
        }
    }

    pub fn membership(&self, s: &str) -> bool {
        if s.len() % 3 != 0 || s.len() == 0 {
            return false;
        }

        let n = s.len();
        let part = n / 3;
        s.get(0..part) == Some(&"a".repeat(part))
            && s.get(part..2 * part) == Some(&"b".repeat(part))
            && s.get(2 * part..n) == Some(&"c".repeat(part))
    }
}