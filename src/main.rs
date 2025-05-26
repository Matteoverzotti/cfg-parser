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

    fn derive(&self, s: &str, depth: u8) -> Option<Vec<String>> {
        if s.len() == 0 {
            return Some(vec!["S".to_string()]);
        }
        let first: Option<&u8> = s.as_bytes().first();
        let last: Option<&u8> = s.as_bytes().last();

        if first == Some(&b'a') && last == Some(&b'b') {
            // Call the function recursively to s[1..s.len() - 1]
            let mut next = self.derive(s.get(1..s.len() - 1).unwrap_or(""), depth + 1)?;

            if depth != 0 {
                next.push(format!("a{}b", next.last().unwrap()));
            } else {
                next.push(s.to_string());
            }
            return Some(next);
        }
        None
    }

    fn membership(&self, s: &str) -> bool {
        // We can use the derive function implemented earlier
        self.derive(s, 0).is_some()
    }
}

fn main() {
    let cfg = CFG::build();
    println!("{:#?}", cfg);

    let random_strings = cfg.generate_random_string(10, 10);
    for s in random_strings {
        println!("{}", s);
    }

    let test_string: &str = "aaabbb";
    if let Some(derivation) = cfg.derive(test_string, 0) {
        println!("Derivation for {:?}: {:?}", test_string, derivation);
    } else {
        println!("No derivation found for {:?}", test_string);
    }

    println!("Membership test for {:?}: {}", test_string, cfg.membership(test_string));
}
