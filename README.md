# Rust Programming - Learning Hub

## 📚 Overview

This is the main directory for all Rust learning projects and courses. Since this folder already exists, we'll use it as a container for different Rust courses and projects. Each course or learning path gets its own subfolder, and within each folder, individual Cargo projects are created for lessons, chapters, or exercises.

## 🗂️ Directory Structure

```
~/code/rust_programming/           # This current directory
├── README.md                      # This file - main documentation
├── .git/                          # Git repository (already initialized)
├── educative_rust_course/         # Educative.io course projects
│   ├── 01_hello_world/           # Each is a separate Cargo project
│   ├── 02_variables/
│   ├── 03_data_types/
│   └── ...
├── rust_book/                     # "The Rust Programming Language" book
│   ├── ch01_getting_started/
│   ├── ch02_guessing_game/
│   └── ...
├── rustlings_exercises/           # Rustlings practice exercises
│   ├── variables/
│   ├── functions/
│   └── ...
├── personal_projects/             # Your own Rust projects
│   ├── calculator/
│   ├── todo_cli/
│   └── ...
└── advent_of_code_2024/          # Advent of Code challenges
    ├── day01/
    ├── day02/
    └── ...
```

**Note**: This directory (`~/code/rust_programming/`) is NOT a Cargo project itself - it's a container for organizing multiple Rust learning paths and projects.

## 🚀 Getting Started

### Step 1: Create a Course/Project Folder

```bash
cd ~/code/rust_programming
mkdir educative_rust_course
cd educative_rust_course
```

### Step 2: Create Individual Cargo Projects

For each lesson, chapter, or exercise, create a new Cargo project:

```bash
# Example: Creating projects for Educative.io course
cargo new 01_hello_world
cargo new 02_variables
cargo new 03_data_types
# ... and so on
```

### Step 3: Work on a Specific Project

```bash
cd 01_hello_world
nvim src/main.rs  # Edit your code
cargo run         # Run the project
```

## 📖 Example: Setting Up Educative.io Rust Course

### Create the Course Directory
```bash
cd ~/code/rust_programming
mkdir educative_rust_course
cd educative_rust_course
```

### Create Projects for Each Topic
```bash
# Module 1: Getting Started
cargo new 01_hello_world
cargo new 02_variables_mutability
cargo new 03_basic_types

# Module 2: Control Flow
cargo new 04_if_expressions
cargo new 05_loops
cargo new 06_match_expressions

# Module 3: Ownership
cargo new 07_ownership_basics
cargo new 08_borrowing_references
cargo new 09_slices

# Continue creating projects as you progress...
```

## 🛠️ Useful Commands

### Navigation Shortcuts
Add these to your `.zshrc` for quick navigation:

```bash
# Rust learning directory aliases
alias rust_learn='cd ~/code/rust_programming'
alias rust_edu='cd ~/code/rust_programming/educative_rust_course'
alias rust_book='cd ~/code/rust_programming/rust_book'
alias rust_proj='cd ~/code/rust_programming/personal_projects'
```

### Cargo Commands Quick Reference

```bash
# Create new project
cargo new project_name

# Build and run
cargo run

# Just build
cargo build

# Build optimized version
cargo build --release

# Check for errors (faster than build)
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Update dependencies
cargo update

# Add a dependency
cargo add crate_name

# Watch and auto-run on changes
cargo watch -x run
```

## 📚 Suggested Course Folders

### 1. Educative.io Courses
```bash
mkdir educative_rust_course
mkdir educative_rust_advanced
mkdir educative_game_dev_rust
```

### 2. Official Rust Resources
```bash
mkdir rust_book           # The Rust Programming Language book
mkdir rust_by_example     # Rust by Example
mkdir rustlings_exercises # Rustlings exercises
```

### 3. Online Courses
```bash
mkdir udemy_rust_course
mkdir coursera_rust
mkdir youtube_tutorials
```

### 4. Practice & Challenges
```bash
mkdir leetcode_rust
mkdir advent_of_code_2024
mkdir exercism_rust
mkdir codewars_rust
```

### 5. Personal Projects
```bash
mkdir personal_projects
mkdir experiments
mkdir mini_projects
```

## 📝 Project Naming Conventions

Use consistent naming for easy navigation:

```bash
# For course chapters/lessons
01_hello_world
02_variables
03_data_types

# For exercises
ex01_temperature_converter
ex02_fibonacci
ex03_guessing_game

# For projects
project_calculator
project_todo_cli
project_web_server
```

## 🎯 Learning Path Recommendations

### Beginner Path
1. Start with `educative_rust_course/`
2. Follow along with `rust_book/`
3. Practice with `rustlings_exercises/`

### Intermediate Path
1. Build projects in `personal_projects/`
2. Solve challenges in `advent_of_code_2024/`
3. Try `exercism_rust/` for mentored practice

### Advanced Path
1. Contribute to open source
2. Build complex applications
3. Explore systems programming

## 🔗 Resources

### Official Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Online Courses
- [Educative.io Rust Courses](https://www.educative.io/courses/rust-programming-language)
- [Udemy Rust Courses](https://www.udemy.com/topic/rust-programming-language/)

### Practice Platforms
- [Exercism](https://exercism.org/tracks/rust)
- [LeetCode](https://leetcode.com/)
- [Advent of Code](https://adventofcode.com/)

## 💡 Tips

1. **Keep projects small and focused** - One concept per project
2. **Name projects descriptively** - Use numbers for ordering
3. **Document your learning** - Add README files to projects
4. **Commit regularly** - Track your progress with git
5. **Experiment freely** - Create an `experiments/` folder for trying things out

## 📄 Notes

This structure allows you to:
- Keep different courses/resources organized
- Easily switch between learning materials
- Track progress in each course separately
- Maintain a clean workspace for all Rust learning

Remember: Each folder in `~/code/rust_programming/` is a container for related Cargo projects, not a Cargo project itself!

---
*Happy Learning! 🦀*