## Introduction

This repository welcomes contributions of all kinds, including:

- Issues for bugs, feature requests, or performance issues
- Documentation improvements
- PR submissions
- PR reviews

## Working with this project

### Setting Up Your Build Environment

To work on this project you need to have a Rust toolchain installed. ([Official installer](https://www.rust-lang.org/tools/install))

Make sure your Rust toolchain is up-to-date to take advantage of the latest language features.

```bash
rustup update stable
```

This project uses some unstable formatting options so you also need a nightly toolchain as well.

```bash
# Install if you don't already have a nightly toolchain
rustup toolchain install nightly

# Update if you already have a nightly toolchain
rustup update nightly
```

### How to compile

This is a standard cargo project with workspaces. To build the entire project simply run:

```bash
cargo build
```

You can also compile specific workspaces:

```bash
cd raekna-parser && cargo build
```

**For Linux users**

Before you build the project run the following to install the dependencies for wgpu:

```bash
sudo apt-get update -y -qq
sudo add-apt-repository ppa:oibaf/graphics-drivers -y
sudo apt-get update -y -qq
sudo apt install -y libegl1-mesa libgl1-mesa-dri libxcb-xfixes0-dev mesa-vulkan-drivers
```

### Running the tests

Tests can be run with the standard `cargo test` command:

```bash
# run all tests.
cargo test

# run only tests for the raekna-compute crate
cargo test -p raekna-compute
```

### Code Formatting

This project uses `rustfmt` to check code formatting. There is a `rustfmt.toml` file that specifies some extra rules around formatting imports. Unfortunately the formatting rules require a nightly toolchain.

To check formatting and linter issues run:

```bash
cargo +nightly fmt --all -- --check
```

### Clippy Lints

Clippy is a hard requirement for this project and the CI will enforce that there are no clippy errors or warnings. If you think clippy is producing a false positive add an exclusion whose scope is as narrow as possible.

To check for clippy warnings run:

```bash
cargo clippy
```

## Contributing 

### Opening an issue 

Before opening an issue please make sure there is no existing issue covering the same issue or feature request. If you still need to open a new issue make sure to follow the issue template.

### Submitting a PR

Before submitting the PR make sure that all checks (`fmt`, `clippy`, `build`, `test`) are running locally.

When opening the PR, please follow the PR template.