> ## day 1 \(15-01-26\)

### Basic Installation
- installed rustup instead of rust becuase rustup helps in management of different version of rust
- rustup is a version management tool
- cmd: rustup update : checkes for available update if present
- every tool is installed in ~/.cargo/bin including rustc, cargo and rustup

> ### Cargo : rust build tool and package manager
- cmd : `cargo build` : builds the project
- cmd : `cargo run` : run the project and checks for rebuits 
- cmd : `cargo doc` : generates documentation for project
- cmd : `cargo --version` : return the version of cargo
- cmd : `cargo new <project_name>` : generates initial project structure in new folder
- cmd : `cargo init <project_ref>` : generates initial project structue in existing folder
- cmd : `cargo add <crate>` : In rust packages is refered as crate
- cmd : `cargo check` : doesn't compiles the project just checks the project can be compiled or not, it makes sure that the project compilies without compling it as compiling it would take more time.

> ## day 2 \(16-01-26\)

> - src : [root/learnings/guessing_game](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/guessing_game "link to folder or guessing_game")
  - completed guessing game and developed an understanding about how rust implements error handling and the basic control flow and logic building in rust.
  - developed basic understanidg about keywords: let, match and mut.

> - pratially completed variables and mutability
  - src : [root/learnings/basic_concepts](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/basic_concepts "link to folder or guessing_game")
  - checkout [BASIC_CONCEPT.md](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/basic_concepts/BASIC_CONCEPT.md "link to detailed guide for basic concept") for more details.

> ## day 3 \(17-01-26\)
  - completed variables and mutability.
  - completed data types in rust.
  - checkout [BASIC_CONCEPT.md](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/basic_concepts/BASIC_CONCEPT.md "link to detailed guide for basic concept") for more details.

> ## day 4 \(18-01-26\)
  - completed functions and learned about expressions and statements.
  - comments is rust single line and multiline comments to improve readibility of code for developers
  - control flow intro to branching: if, else, else if and loops: loop, while, for
  - checkout [BASIC_CONCEPT.md](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/basic_concepts/BASIC_CONCEPT.md "link to detailed guide for basic concept") for more details.

> ## day 5 \(19-01-26\)
  - on the rust ownership and borrowing model 
  - how rust handels stack and heap allocation and deallocation.
  - checkout [OWNERSHIP.md](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/ownership/OWNERSHIP.md "link to detailed guide for ownership model") for more details.

> ## day 6 \(20-01-26\)
  - how references and borrowing works in rust.
  - how rust references and how we can manually reference the data in rust using `*` and `&` operators.
  - checkout [OWNERSHIP.md](https://github.com/Vishal-KrRana/rusty/tree/main/learnings/ownership/OWNERSHIP.md "link to detailed guide for ownership model") for more details.
