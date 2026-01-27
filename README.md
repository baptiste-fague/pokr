# POKR

### Principle

This project aims at building a poker AI.

The first component of the project is a poker game engine for the AI to be able to play.

### Build python bindings

- Create venv
- Activate venv
```
.\.env\Scripts\activate
```
- Compile binding
```
maturin develop
```
- Generate pyi stubs
```
cargo run --bin stub_gen
```