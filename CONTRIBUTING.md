# Contributing to toolchest

Thanks for your interest in contributing! Please follow these guidelines to make the process smooth.

## Getting Started
- Fork and clone the repo
- Create a branch: `git checkout -b feature/your-change`
- Run tests: `cargo test --all-features`
- Run lints: `cargo fmt --all` and `cargo clippy --all-features -- -D warnings`

## Code Style
- Edition 2021, MSRV 1.70
- No `unsafe` code
- Zero runtime dependencies
- Document every public item

## Commit Messages
- Use Conventional Commits (e.g., `feat:`, `fix:`, `docs:`, `chore:`)

## Pull Requests
- Include tests for new functionality
- Update docs when needed
- Ensure CI is green

## License
By contributing, you agree your contributions will be dual-licensed under MIT or Apache-2.0.
