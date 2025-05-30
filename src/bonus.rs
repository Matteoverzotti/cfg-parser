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

impl BonusCFG {
    pub fn build() -> Self {
        let productions = vec![
            Production {
                lhs: Symbol::NonTerminal('S'),
                rhs: vec![Symbol::Terminal('a'), Symbol::NonTerminal('A'),
                        Symbol::Terminal('b'), Symbol::NonTerminal('B'),
                        Symbol::Terminal('c'), Symbol::NonTerminal('C')]
            },
            Production {
                lhs: Symbol::NonTerminal('A'),
                rhs: vec![Symbol::Terminal('a'), Symbol::NonTerminal('A')]
            },
            Production {
                lhs: Symbol::NonTerminal('A'),
                rhs: vec![]
            },
            Production {
                lhs: Symbol::NonTerminal('B'),
                rhs: vec![Symbol::Terminal('b'), Symbol::NonTerminal('B')]
            },
            Production {
                lhs: Symbol::NonTerminal('B'),
                rhs: vec![]
            },
            Production {
                lhs: Symbol::NonTerminal('C'),
                rhs: vec![Symbol::Terminal('c'), Symbol::NonTerminal('C')]
            },
            Production {
                lhs: Symbol::NonTerminal('C'),
                rhs: vec![]
            },
        ];
        BonusCFG { 
            _nonterminals: vec![Symbol::NonTerminal('S'), Symbol::NonTerminal('A'), 
                               Symbol::NonTerminal('B'), Symbol::NonTerminal('C')], 
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

// The language L = {a^n b^n c^n | n >= 1} is not Context-Free (proof by Pumping Lemma)
// So no CFG can be built for it
// However, we can do a close equivalent which is exactly present in this file
// S -> aAbBcC
// A -> aA | ε
// B -> bB | ε
// C -> cC | ε
// This grammar correctly generates all words in the language L
// But unfortunately it can generate other words as well
// More specifically, it generates a+b+c+.