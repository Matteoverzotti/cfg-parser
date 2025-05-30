mod bonus;
use bonus::BonusCFG;
use rand::seq::SliceRandom;
use std::process::exit;

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
    _nonterminals: Vec<Symbol>,
    _terminals: Vec<Symbol>,
    _start_symbol: Symbol,
    _productions: Vec<Production>,
}

// S -> aSb | ε
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
            _nonterminals: vec![Symbol::NonTerminal('S')],
            _terminals: vec![Symbol::Terminal('a'), Symbol::Terminal('b')],
            _start_symbol: Symbol::NonTerminal('S'),
            _productions: productions,
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

fn print_usage() {
    println!("Usage: cargo run <cfg_type> <cfg_method>");
    println!("cfg_type: 'cfg' or 'bonus'");
    println!("cfg_method for 'cfg': 'build', 'generate', 'derive', 'membership'");
    println!("cfg_method for 'bonus': 'build', 'membership'");
    exit(1);
}

fn main() {
    let cfg_type = std::env::args().nth(1);
    let cfg_method = std::env::args().nth(2);

    if cfg_type == None || cfg_method == None {
        print_usage();
    }

    // There is surely a cleaner way to implement this
    // But I'm lazy
    if cfg_type == Some(String::from("cfg")) {
        let cfg = CFG::build();
        let strings = ["aaabbb", "aabbb", "a", "ba"];

        match cfg_method.unwrap().as_str() {
            "build" => println!("{:#?}", cfg),
            "generate" => {
                let random_strings = cfg.generate_random_string(10, 10);
                for s in random_strings {
                    println!("{}", s);
                }
            },
            "derive" => {
                for s in strings {
                    if let Some(derivation) = cfg.derive(s, 0) {
                        println!("Derivation for {:?}: {:?}", s, derivation);
                    } else {
                        println!("No derivation found for {:?}", s);
                    }
                }
            },
            "membership" => {
                for s in strings {
                    println!("Membership test for {:?}: {}", s, cfg.membership(s));
                }
            },
            _ => print_usage(),
        }
    } else if cfg_type == Some(String::from("bonus")) {
        let bonus_cfg: BonusCFG = BonusCFG::build();
        let strings = ["bc", "abc", "aabbcc", "acbca", ""];
        match cfg_method.unwrap().as_str() {
            "build" => println!("{:#?}", bonus_cfg),
            "membership" => {
                for s in strings {
                    println!("Membership test for {:?}: {}", s, bonus_cfg.membership(s));
                }
            },
            _ => print_usage(),
        }
    } else {
        print_usage();
    }

}
