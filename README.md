# CFG Parser

This project implements a Context-Free Grammar parser in Rust, capable of building context-free grammars, generating words based off of them, deriving words and testing membership. It can support simple grammars.

## Features

- **CFG Building**: Builds a CFG object from a given setup.
- **Word generating**: Generates words that satisfy the given grammar.
- **Word derivation**: Shows the derivation for a given word.
- **Word membership**: Tests whether a words belongs to the given CFG.

## Project Structure

```plaintext
cfg-parser/ 
├── .gitignore 
├── Assignment__3___Limbaje_Formale_și_Automate.json 
├── Cargo.lock 
├── Cargo.toml 
├── README.md
└── src/ 
    ├── bonus.rs 
    └── main.rs
```

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

## Building the Project

To build the project, run:

```bash
cargo build
```

## Running the program

```bash
Usage: cargo run <cfg_type> <cfg_method>
cfg_type: 'cfg' or 'bonus'
cfg_method for 'cfg': 'build', 'generate', 'derive', 'membership'
cfg_method for 'bonus': 'build', 'membership'
```

### Simple CFG

```bash
╭╴ matteoverz on numenor at …/cfg-parser via  main ( ⇡6) is 📦 v0.1.0 via 🦀 v1.84.1 
╰─ cargo run cfg build  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfg-parser cfg build`
CFG {
    _nonterminals: [
        NonTerminal(
            'S',
        ),
    ],
    _terminals: [
        Terminal(
            'a',
        ),
        Terminal(
            'b',
        ),
    ],
    _start_symbol: NonTerminal(
        'S',
    ),
    _productions: [
        Production {
            lhs: NonTerminal(
                'S',
            ),
            rhs: [
                Terminal(
                    'a',
                ),
                NonTerminal(
                    'S',
                ),
                Terminal(
                    'b',
                ),
            ],
        },
        Production {
            lhs: NonTerminal(
                'S',
            ),
            rhs: [],
        },
    ],
}

╭╴ matteoverz on numenor at …/cfg-parser via  main ( ⇡6) is 📦 v0.1.0 via 🦀 v1.84.1 
╰─ cargo run cfg generate
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfg-parser cfg generate`
aaabbb

aabb
ab
aabb
aaabbb
ab
aaabbb
ab
aaabbb

╭╴ matteoverz on numenor at …/cfg-parser via  main ( ⇡6) is 📦 v0.1.0 via 🦀 v1.84.1 
╰─ cargo run cfg derive
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfg-parser cfg derive`
Derivation for "aaabbb": ["S", "aSb", "aaSbb", "aaabbb"]
No derivation found for "aabbb"
No derivation found for "a"
No derivation found for "ba"

╭╴ matteoverz on numenor at …/cfg-parser via  main ( ⇡6) is 📦 v0.1.0 via 🦀 v1.84.1 
╰─ cargo run cfg membership
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfg-parser cfg membership`
Membership test for "aaabbb": true
Membership test for "aabbb": false
Membership test for "a": false
Membership test for "ba": false

```

### Bonus CFG

```bash
╭╴ matteoverz on numenor at …/cfg-parser via  main ( ⇡6) is 📦 v0.1.0 via 🦀 v1.84.1 
╰─ cargo run bonus build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfg-parser bonus build`
BonusCFG {
    _nonterminals: [
        NonTerminal(
            'S',
        ),
        NonTerminal(
            'A',
        ),
        NonTerminal(
            'B',
        ),
        NonTerminal(
            'C',
        ),
    ],
    _terminals: [
        Terminal(
            'a',
        ),
        Terminal(
            'b',
        ),
        Terminal(
            'c',
        ),
    ],
    _start_symbol: NonTerminal(
        'S',
    ),
    _productions: [
        Production {
            lhs: NonTerminal(
                'S',
            ),
            rhs: [
                Terminal(
                    'a',
                ),
                NonTerminal(
                    'A',
                ),
                Terminal(
                    'b',
                ),
                NonTerminal(
                    'B',
                ),
                Terminal(
                    'c',
                ),
                NonTerminal(
                    'C',
                ),
            ],
        },
        Production {
            lhs: NonTerminal(
                'A',
            ),
            rhs: [
                Terminal(
                    'a',
                ),
                NonTerminal(
                    'A',
                ),
            ],
        },
        Production {
            lhs: NonTerminal(
                'A',
            ),
            rhs: [],
        },
        Production {
            lhs: NonTerminal(
                'B',
            ),
            rhs: [
                Terminal(
                    'b',
                ),
                NonTerminal(
                    'B',
                ),
            ],
        },
        Production {
            lhs: NonTerminal(
                'B',
            ),
            rhs: [],
        },
        Production {
            lhs: NonTerminal(
                'C',
            ),
            rhs: [
                Terminal(
                    'c',
                ),
                NonTerminal(
                    'C',
                ),
            ],
        },
        Production {
            lhs: NonTerminal(
                'C',
            ),
            rhs: [],
        },
    ],
}

╭╴ matteoverz on numenor at …/cfg-parser via  main ( ⇡6) is 📦 v0.1.0 via 🦀 v1.84.1 
╰─ cargo run bonus membership
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/cfg-parser bonus membership`
Membership test for "bc": false
Membership test for "abc": true
Membership test for "aabbcc": true
Membership test for "acbca": false
Membership test for "": false
```
