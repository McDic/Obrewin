# Obrewin Framework

A quantitative trading framework made for indie trading in Rust.

- [Obrewin Data Structures](https://crates.io/crates/obrewin-data-structures)
- [Obrewin Network](https://crates.io/crates/obrewin-network)
- [Obrewin Utils](https://crates.io/crates/obrewin-utils)

## User Guide

### Dependencies

- Python 3.11+
- Rust 1.72+

Add following dependencies in your `Cargo.toml`.
Every crate here will be version-synchronized automatically,
so use same versions if you are using multiple crates from this repository.

```toml
[dependencies]
obrewin-data-structures = { version = "*" }
obrewin-network = { version = "*" }
obrewin-utils = { version = "*" }
```

## Developer Guide

Main development is going under WSL 2 Ubuntu 22.04 LTS.

### Construct Python environment

Make your Python virtual environment, then install dependencies by following.

```bash
pip install -r requirements-dev.txt
```

### Configure [`pre-commit`](https://pre-commit.com/)

```bash
pre-commit install
```
