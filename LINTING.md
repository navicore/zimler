# Linting Standards

This project enforces STRICT linting standards. Both local development and CI use the exact same checks via `make lint`.

## Required Checks

All code MUST pass:
1. `cargo fmt --check` - Formatting must be perfect
2. `cargo clippy` with pedantic/nursery rules - No warnings allowed
3. `cargo check` - Type checking must pass

## Before Committing

Always run:
```bash
make lint
```

If there are issues, try:
```bash
make fix
```

## Common Issues & Fixes

### Clippy Pedantic Issues
- Missing `#[must_use]` attributes on constructors
- Missing `Default` implementations
- Non-const functions that could be const
- Unnecessary type conversions
- Missing documentation

### Quick Fixes

For auto-fixable issues:
```bash
cargo fmt --all
cargo clippy --fix --allow-staged
```

## CI Integration

GitHub Actions runs exactly:
```bash
make ci
```

Which is identical to:
```bash
make lint test build
```

No surprises - what passes locally will pass in CI.

## Setting Up Pre-commit Hooks

To ensure you never commit unlinted code:
```bash
make install-hooks
```

This installs a git pre-commit hook that runs `make lint` automatically.