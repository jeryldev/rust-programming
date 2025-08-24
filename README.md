# Rust Programming - Learning Hub

## 📚 Overview

Central directory for all Rust learning projects. Organized by course/resource, with flexibility to use either single projects with multiple binaries (for exercises) or separate projects (for larger applications).

## 🚀 Prerequisites & Setup

### Install Rust

Install Rust via [rustup](https://www.rust-lang.org/tools/install) (the official Rust installer):

**Linux/macOS/Unix:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run [rustup-init.exe](https://www.rust-lang.org/tools/install)

After installation, restart your terminal and verify:
```bash
rustc --version
cargo --version
```

### Clone & Run This Repository

```bash
# Clone the repository
git clone <your-repo-url>
cd rust_programming

# Navigate to an existing project
cd educative_rust_programming_language_course

# Run a specific program
cargo run --bin hello_world

# Or list all available programs
ls src/bin/
```

### Optional: Install Development Tools

```bash
# Code formatter
rustup component add rustfmt

# Linter
rustup component add clippy

# Language server (required for IDE support)
rustup component add rust-analyzer

# Auto-rerun on file changes
cargo install cargo-watch
```

### Neovim Setup (LazyVim)

For Neovim users with LazyVim, enable the Rust language extra:

```vim
:LazyExtras
```
Then select and enable `lang.rust` extra. This will install:
- **rustaceanvim** - Enhanced Rust plugin (successor to rust-tools.nvim)
- **crates.nvim** - Cargo.toml LSP support
- **Treesitter** - Rust and RON syntax highlighting
- **Codelldb** - Debugger support via Mason

The Rust extra requires `rust-analyzer` to be installed (see above).

## 🗂️ Directory Structure

```
~/code/rust_programming/                              # This current directory
├── .gitignore                                       # Git ignore file
├── README.md                                         # This file
├── educative_rust_programming_language_course/      # Educative.io course (Cargo project)
│   ├── Cargo.toml                                  # Project configuration
│   ├── Cargo.lock                                  # Dependency lock file
│   └── src/
│       ├── main.rs                                 # Optional default program
│       └── bin/                                    # ⚠️ ALL exercise files go here
│           └── hello_world.rs                     # Example exercise
├── rust_book/                                       # "The Rust Programming Language" book (future)
├── rustlings/                                       # Rustlings exercises (future)
├── personal_projects/                              # Your own Rust applications (future)
└── experiments/                                    # Quick tests and explorations (future)

```

## 🚀 Two Workflows

### Workflow A: Multiple Binaries in One Project (For Courses/Exercises)

Perfect when working through tutorials with many small programs:

```bash
# For new projects:
cd ~/code/rust_programming
cargo new project_name
cd project_name
mkdir src/bin

# For existing projects (like educative_rust_programming_language_course):
cd ~/code/rust_programming/educative_rust_programming_language_course

# IMPORTANT: Create programs in src/bin/ NOT in src/
# Files in src/bin/ are automatically recognized as binary targets
nvim src/bin/01_hello_world.rs  # ✅ Correct location
nvim src/bin/02_variables.rs    # ✅ Correct location
# NOT: nvim src/01_hello_world.rs ❌ Wrong - causes rust-analyzer warnings

# Write your code (example for 01_hello_world.rs)
cat > src/bin/01_hello_world.rs << 'EOF'
fn main() {
    println!("Hello, World!");
    println!("Welcome to Rust Programming!");
}
EOF

# Run programs
cargo run --bin 01_hello_world
cargo run --bin 02_variables
rust_run 1  # Using helper function (see below)
```

**Note**: Always create exercise files in `src/bin/` directory. Files placed directly in `src/` will show rust-analyzer warnings about not being included in the module tree.

### Workflow B: Separate Projects (For Real Applications)

For substantial projects with their own dependencies:

```bash
cd ~/code/rust_programming/personal_projects
cargo new todo_cli
cargo new web_server
```

## 🛠️ Shell Helper Functions

Add to your `.zshrc`:

```bash
#=============================================================================
#                     Rust Learning Helper Functions
#=============================================================================

# Navigation
alias rust_learn='cd ~/code/rust_programming'
alias rust_edu='cd ~/code/rust_programming/educative_rust_programming_language_course'
alias rust_proj='cd ~/code/rust_programming/personal_projects'

# List all binaries in current project
rust_list() {
    if [[ -d "src/bin" ]]; then
        echo "Available programs:"
        ls -1 src/bin/*.rs 2>/dev/null | xargs -n1 basename | sed 's/\.rs$//' | nl
    else
        echo "No src/bin directory found."
    fi
}

# Run by name or number
rust_run() {
    if [ -z "$1" ]; then
        rust_list
        echo "\nUsage: rust_run <name or number>"
        return 1
    fi

    if [[ "$1" =~ ^[0-9]+$ ]]; then
        local file=$(ls -1 src/bin/*.rs 2>/dev/null | sed -n "${1}p" | xargs -n1 basename | sed 's/\.rs$//')
        [ -z "$file" ] && echo "No file #$1" && return 1
        cargo run --bin "$file"
    else
        cargo run --bin "$1"
    fi
}

# Create new binary with template (creates in src/bin/)
rust_new() {
    [ -z "$1" ] && echo "Usage: rust_new <filename>" && return 1

    local filename="${1%.rs}.rs"
    mkdir -p src/bin  # Ensures src/bin/ exists

    # Creates file in src/bin/ NOT src/
    cat > "src/bin/$filename" << EOF
fn main() {
    println!("=== ${1%.rs} ===\n");

    // Your code here
}
EOF

    echo "Created: src/bin/$filename"  # Note: in src/bin/
    echo "Run with: cargo run --bin ${filename%.rs}"
}

# Watch & auto-run
rust_watch() {
    [ -z "$1" ] && echo "Usage: rust_watch <program>" && return 1
    cargo watch -x "run --bin $1"
}
```

## ⚡ Quick Commands

### Shell Commands

```bash
# Navigation
rust_learn              # Go to main Rust directory
rust_edu                # Go to Educative course
rust_proj               # Go to personal projects

# Working with programs
rust_list               # Show all programs (numbered)
rust_run 1              # Run program #1
rust_run hello_world    # Run by name
rust_new 05_loops       # Create new program
rust_watch 05_loops     # Auto-rerun on changes

# Standard Cargo
cargo build             # Build project
cargo check             # Quick error check
cargo test              # Run tests
cargo fmt               # Format code
cargo clippy            # Lint code
cargo doc --open        # Generate docs
```

### Neovim Rust Commands

```vim
:RustLsp run            # Run current binary
:RustLsp debuggables    # Debug current binary
:RustLsp testables      # Run tests
:RustLsp expandMacro    # Expand macro under cursor
:RustLsp openCargo      # Open Cargo.toml
:RustLsp openDocs       # Open docs for symbol
:RustLsp parentModule   # Go to parent module
```

**Note**: The keymaps like `<Space>rr` might need to be configured manually. Use `:RustLsp` commands directly for now.

## 📖 Example: Working with Existing Projects

```bash
# 1. Navigate to existing course project
cd ~/code/rust_programming/educative_rust_programming_language_course

# 2. Create new lesson files
rust_new 01_hello_world
rust_new 02_variables
rust_new 03_data_types
rust_new 04_control_flow
rust_new 05_ownership

# 3. Write code (example: edit 01_hello_world.rs)
cat > src/bin/01_hello_world.rs << 'EOF'
fn main() {
    println!("=== 01_hello_world ===\n");

    // Basic output
    println!("Hello, World!");

    // Using variables
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // Multiple values
    let language = "Rust";
    let year = 2025;
    println!("Learning {} in {}", language, year);
}
EOF

# 4. Run and test
rust_list               # See all programs
rust_run 1              # Run first program (01_hello_world)
rust_run 01_hello_world # Or run by name
rust_watch 01_hello_world # Auto-rerun on changes
```

## 🎯 Learning Path

### Beginner

1. Start with `educative_rust_programming_language_course/` - fundamentals
2. Try `rustlings/` - guided practice (when created)
3. Read along with `rust_book/` - official book

### Intermediate

1. Build in `personal_projects/` - real applications
2. Solve `advent_of_code/` - algorithmic challenges
3. Contribute to open source

### Advanced

1. Systems programming projects
2. Async/concurrent applications
3. Embedded Rust

## 🔗 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Educative.io Rust](https://www.educative.io/courses/rust-programming-language)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

## 💡 Tips

1. **Use numbered prefixes** (`01_`, `02_`) for ordered lessons
2. **One concept per file** when learning
3. **Commit progress** regularly to track learning
4. **Read compiler errors** - Rust's are excellent
5. **Use `cargo clippy`** for style suggestions

---

_Happy Learning! 🦀_

