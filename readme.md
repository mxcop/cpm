# C/C++ Package Manager (cpm)
<sup>Concept Repo</sup>

CPM is a concept for a C/C++ package manager which attempts to mimic the user experience of NPM.<br>
To make C/C++ projects easier to setup and maintain for everyone.

## Project structure
```graphql
./<project>/* 
  │
  ├─ .cpm       - # C/C++ packages
  ├─ target     - # Build directory
  │
  ├─ inc/*      - # Your header files (.h/.hpp)
  │  └─ ...
  ├─ src/*      - # Your code files (.c/.cpp)
  │  ├─ main.c  - # Entry point
  │  └─ ...
  │
  └─ cpm.toml   - # CPM project config
```

## Commands
```bash
# Create a new project
$ cpm create <project-name>

# Install and remove packages
$ cpm install <package-name>...
$ cpm remove <package-name>...

# Run and build your project
$ cpm run --
$ cpm build --
```

## Config
```toml
[package]
name = "my-package"
version = "0.0.0"
authors = ["John Doe <john.doe@gmail.com>"]
license = "MIT"

[compiler]
name = "clang"
version = "14.0.0"
arguments = ["--"]

[run]    # optional
arguments = ["--"]

[build]  # optional
arguments = ["--"]

[dependencies]
package = "0.0.0"
```

## Build with
The CLI would be build using [Rust-lang](https://www.rust-lang.org/).<br>
All things like building, running, and package/library management would be handled by the CLI.<br>
No need to touch either the compiler or cmake/make.