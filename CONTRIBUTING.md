# CONTRIBUTING.md

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please respect all contributors.

## How to Contribute

### Reporting Bugs

- Use the GitHub issue tracker
- Describe the bug clearly
- Provide steps to reproduce
- Include system information

### Suggesting Features

- Use the GitHub issue tracker with label "enhancement"
- Describe the feature and its benefits
- Explain the expected behavior

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Ensure all tests pass (`cargo test && pytest`)
6. Commit with clear messages
7. Push and create a PR

### Development Setup

See [DEVELOPMENT.md](docs/DEVELOPMENT.md)

## Code Standards

### Rust

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write tests for new functionality
- Document public APIs

### Python

- Run `black` and `isort` for formatting
- Run `flake8` and `mypy` for linting
- Write tests with pytest
- Follow PEP 8

### TypeScript/JavaScript

- Run `eslint` and `prettier`
- Follow React best practices
- Add JSDoc comments

## Commit Guidelines

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Type:** feat, fix, docs, style, refactor, test, chore
**Scope:** core, protocols, ml, api, ui, config

**Example:**

```
feat(protocols): implement Streamr adapter

- Add WebSocket connection handling
- Implement earnings tracking with indexer
- Add comprehensive health checks
- Include configuration templates

Closes #123
```

## Questions?

Feel free to open an issue or reach out to the team.

---

Thank you for contributing! 🚀
