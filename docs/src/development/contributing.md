# Contributing to Balinese Calendar

Thank you for your interest in contributing to the Balinese Calendar project! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust 1.70.0 or later (MSRV)
- Git
- mdBook (for documentation): `cargo install mdbook`

### Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/balinese-calendar.git
   cd balinese-calendar
   ```
3. Create a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Development Workflow

1. **Make your changes** following the coding standards below
2. **Run tests** to ensure everything works:
   ```bash
   cargo test --all-features
   ```
3. **Check formatting and linting**:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   ```
4. **Run security audit**:
   ```bash
   cargo audit
   ```
5. **Build documentation** (if you changed docs):
   ```bash
   cd docs && mdbook build
   ```
6. **Commit your changes** with a descriptive message
7. **Push to your fork** and create a pull request

## Coding Standards

### Code Style

- Follow standard Rust formatting (enforced by `rustfmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and reasonably sized
- Avoid unnecessary complexity

### Testing

- Write unit tests for new functionality
- Add integration tests for complex features
- Ensure all tests pass before submitting
- Aim for good test coverage (target: 80%+)
- Test edge cases and error conditions

### Documentation

- Document all public APIs with `///` comments
- Include examples in documentation where helpful
- Update README.md if adding new features
- Update the mdBook documentation for user-facing changes
- Use clear, concise language

## Pull Request Process

1. **Ensure CI passes** - all checks must be green
2. **Update documentation** if you've changed APIs
3. **Add tests** for new functionality
4. **Write a clear PR description** explaining:
   - What changes you made
   - Why you made them
   - How to test the changes
5. **Link any relevant issues**

### PR Requirements

- [ ] All CI checks pass
- [ ] Tests added for new functionality
- [ ] Documentation updated if needed
- [ ] No breaking changes (unless discussed)
- [ ] Commit messages are clear and descriptive
- [ ] Code follows project style guidelines

## Code Review

All submissions require review. We use GitHub pull requests for this purpose. Reviewers will check for:

- Code quality and style
- Test coverage
- Documentation completeness
- Performance implications
- Security considerations
- Compatibility with existing code

## Types of Contributions

### Bug Reports

When reporting bugs:

1. **Search existing issues** first
2. **Use the bug report template** when available
3. **Provide clear reproduction steps**
4. **Include relevant system information**
5. **Attach error messages and logs**

### Feature Requests

When requesting features:

1. **Check if it's already been requested**
2. **Explain the use case** clearly
3. **Describe the expected behavior**
4. **Consider backwards compatibility**

### Documentation Improvements

Documentation contributions are highly valued! You can help by:

- Fixing typos and grammar
- Adding examples
- Clarifying confusing sections
- Translating documentation
- Adding diagrams or illustrations

### Code Contributions

Code contributions should:

- Solve a real problem
- Be well-tested
- Follow project conventions
- Include documentation
- Not break existing functionality

## Development Guidelines

### Adding New Features

1. **Discuss first** - Open an issue to discuss major changes
2. **Start small** - Break large features into smaller PRs
3. **Write tests** - Test-driven development is encouraged
4. **Document** - Update both code docs and user guide
5. **Benchmark** - Consider performance implications

### Fixing Bugs

1. **Reproduce** - Ensure you can reproduce the bug
2. **Write a test** - Add a test that fails with the bug
3. **Fix** - Implement the fix
4. **Verify** - Ensure the test now passes
5. **Check for regressions** - Run full test suite

### Performance Improvements

1. **Measure first** - Use benchmarks to establish baseline
2. **Profile** - Identify actual bottlenecks
3. **Optimize** - Make targeted improvements
4. **Benchmark again** - Verify improvements
5. **Document** - Explain the optimization

## Security

If you discover a security vulnerability:

1. **Do NOT open a public issue**
2. **Email the maintainers directly**
3. **Provide detailed information**
4. **Wait for acknowledgment** before public disclosure

## Community Guidelines

- Be respectful and inclusive
- Assume good intentions
- Provide constructive feedback
- Help others learn and grow
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (Apache-2.0).

## Getting Help

- **Documentation**: Check the [user guide](../guide/getting-started.md)
- **Issues**: Search existing issues or open a new one
- **Discussions**: Use GitHub Discussions for questions
- **Chat**: Join our community chat (if available)

## Recognition

Contributors are recognized in:

- The project README
- Release notes
- Git commit history
- Special thanks in major releases

Thank you for contributing to the Balinese Calendar project! 🙏
