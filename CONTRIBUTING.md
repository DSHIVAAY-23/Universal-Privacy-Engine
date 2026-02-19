# Contributing to Universal Privacy Engine

## Git identity

Before making commits, set your real name and email:

```bash
git config user.name  "Your Real Name"
git config user.email "your-email@example.com"
```

To make it global (applies to all repos):

```bash
git config --global user.name  "Your Real Name"
git config --global user.email "your-email@example.com"
```

### Rewriting author metadata on recent commits

If you need to fix authorship on already-pushed commits:

```bash
# Interactive rebase — mark commits you want to change with 'reword' or use exec
git rebase -i HEAD~5

# Or, rewrite author on all commits since a specific ref non-interactively:
git rebase HEAD~5 --exec \
  'git commit --amend --no-edit --author="Real Name <email@example.com>"'

# If already pushed, you must force-push:
git push -f origin main
```

> **Warning:** Force-pushing rewrites public history. Coordinate with any
> collaborators before doing this on a shared branch.

---

## Commit message format

Follow [Conventional Commits](https://www.conventionalcommits.org/). Examples:

```
feat(notary): add healthz liveness probe endpoint
fix(notary): correct abi.encodePacked byte layout in hash function
feat(solidity): replace manual ecrecover with OZ ECDSA
test(foundry): add replay-protection and cross-employee isolation tests
ci: add cargo + forge test workflow
docs: add trust model and demo screencast section to README
chore: update .env.example defaults
```

Limit the subject line to **72 characters**.

---

## Running the test suite locally

```bash
# Rust (ECDSA signing, hash correctness, signature roundtrip)
cd core && cargo test --release

# Solidity (valid proof, replay, cross-employee, malformed sig)
cd contracts/oasis && forge test -vvv
```

---

## Security review checklist before opening a PR

- [ ] No private keys or secrets in committed files
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `forge test` passes with no skipped tests
- [ ] New cryptographic logic has a corresponding negative test
- [ ] Sapphire-only guarantees are documented in NatSpec if contract state is involved
