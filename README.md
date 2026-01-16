> # day 1 \(15-01-26\)

# Basic Installation
- installed rustup instead of rust becuase rustup helps in management of different version of rust
- rustup is a version management tool
- cmd: rustup update : checkes for available update if present
- every tool is installed in ~/.cargo/bin including rustc, cargo and rustup

> # Cargo : rust build tool and package manager
- cmd : cargo build : builds the project
- cmd : cargo run : run the project and checks for rebuits 
- cmd : cargo doc : generates documentation for project
- cmd : cargo --version : return the version of cargo
- cmd : cargo new <project_name> : generates initial project structure in new folder
- cmd : cargo init <project_ref> : generates initial project structue in existing folder
- cmd : cargo add <crate> : In rust packages is refered as crate
- cmd : cargo check : doesn't compiles the project just checks the project can be compiled or not, it makes sure that the project compilies without compling it as compiling it would take more time.
