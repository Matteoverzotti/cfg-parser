use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Eq)]
enum Symbol {
    Terminal(char),
    NonTerminal(char),
}

#[derive(Debug, PartialEq, Eq)]
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
                lhs: Symbol::NonTerminal('S'),
                rhs: vec![Symbol::Terminal('a'), Symbol::NonTerminal('S'), Symbol::Terminal('b')]
            },
            Production {
                lhs: Symbol::NonTerminal('S'),
                rhs: vec![] // ε = empty production
            }
        ];
        CFG {
            nonterminals: vec![Symbol::NonTerminal('S')],
            terminals: vec![Symbol::Terminal('a'), Symbol::Terminal('b')],
            start_symbol: Symbol::NonTerminal('S'),
            productions,
        }
    }

    fn generate_random_string(&self, count: usize, max_len: usize) -> Vec<String> {
        let mut result = Vec::new();
        self.backtrack(String::from(""), &mut result, count, max_len);

        // Make the result more funky
        result.shuffle(&mut rand::rng());

        result
    }

    /// Barebones backtracking algorithm, at each step you either do `S -> aSb` or `S -> ε`
    fn backtrack(&self, s: String, result: &mut Vec<String>, count: usize, max_len: usize) {
        if count == 0 || s.len() > max_len {
            return;
        }

        result.push(s.clone());

        let mut new_s = String::from("a");
        new_s.push_str(&s);
        new_s.push('b');

        // 50% chance that S -> aSb
        // 50% chance that S -> ε 
        if rand::random::<f32>() < 0.5 {
            self.backtrack(new_s.clone(), result, count - 1, max_len);
        } else {
            self.backtrack(s, result, count - 1, max_len);
        }
    }
}

fn main() {
    let cfg = CFG::build();
    println!("{:#?}", cfg);

    let random_strings = cfg.generate_random_string(10, 10);
    for s in random_strings {
        println!("{}", s);
    }
}
