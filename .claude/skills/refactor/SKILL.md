---
name: refactor
description: Code quality analysis and refactoring with validation
user-invocable: true
allowed-tools: Read, Grep, Glob, Bash(cargo check*), Bash(cargo clippy*), Bash(cargo test*), Bash(cargo fmt*), Bash(git status*), Bash(git diff*), Bash(git stash*), Bash(wc *), Edit, Write, Task
---

# Refactor

Code quality analysis and incremental refactoring with continuous validation.

Arguments (optional):
- Path or crate: `/refactor crates/app` — limit scope
- No arguments — analyze the entire project

## Process

### Phase 1: Exploration

1. Read CLAUDE.md and doc/ru/PRD.md for project rules and conventions
2. Collect baseline metrics (in parallel):
   - `cargo clippy --workspace --all-targets 2>&1` — warnings count
   - Glob + wc — LOC per crate, files >400 lines
   - `cargo test --workspace 2>&1` — baseline: all tests passing?
3. If tests fail or code doesn't compile — report and stop

### Phase 2: Diagnosis

Analyze code across 8 dimensions. For large scope, use parallel Task agents (subagent_type=Explore) — one per category or group of categories.

**Analysis checklist:**

**Architecture (SOLID)**
- [ ] Files >400 LOC with multiple public types — splitting candidates
- [ ] Modules with >1 responsibility
- [ ] Hard dependencies between modules (concrete types instead of traits)
- [ ] Open/Closed violations: match on types that will be extended

Legitimate exceptions to "1 file = 1 type": Error+ErrorKind pairs, Builder+Target, small private helpers (<30 LOC), related DTO families (Request/Response), typestate marker types, newtype wrapper collections.

**Module coupling / cohesion**
- [ ] Crate dependency graph: unidirectional? (core ← mid ← app)
- [ ] Circular dependencies between modules within a crate
- [ ] Excessive `pub` API — types/functions public without necessity
- [ ] Crate knows implementation details of another crate (leaky abstraction)
- [ ] Changing one module cascades breakage into unrelated modules

**Duplication (DRY)**
- [ ] Repeated code blocks (>5 lines, >2 occurrences)
- [ ] Similar match patterns across different files
- [ ] Copy-pasted error handling

**Dead code**
- [ ] Unused `use`, functions, types, constants
- [ ] Commented-out code
- [ ] Unreachable branches

**Performance**
- [ ] Unnecessary `.clone()` / `.to_string()` / `.to_owned()` where a reference suffices
- [ ] O(n²) loops (nested iterations over collections)
- [ ] Repeated lookups in HashMap/Vec inside a loop
- [ ] Allocations in hot paths

**Error boundaries**
- [ ] Consistent error types at crate boundaries (From / thiserror)
- [ ] `unwrap()` / `expect()` in production code — replace with `?` or explicit handling
- [ ] Lost error context (`.map_err(|_| ...)` without information)
- [ ] Panic instead of Result at input boundaries (user, DB, network)

**Security**
- [ ] SQL queries via string concatenation instead of parameterized
- [ ] Secrets / tokens hardcoded or logged
- [ ] Unvalidated user input (path traversal, XSS)
- [ ] Magic numbers and hardcoded config that should be parameters

**Style and idioms**
- [ ] Inconsistent naming
- [ ] Missing `#[must_use]` on pure functions
- [ ] Over-abstractions with no usage
- [ ] Uncovered edge cases in tests (empty input, errors, overflow)

### Phase 3: Report and prioritization

Output to terminal (do not create files):

```
## Analysis results

| Category       | Found | Critical |
|----------------|-------|----------|
| Architecture   | N     | N        |
| Coupling       | N     | N        |
| Duplication    | N     | N        |
| Dead code      | N     | N        |
| Performance    | N     | N        |
| Errors         | N     | N        |
| Security       | N     | N        |
| Style          | N     | N        |

Baseline: X clippy warnings, Y total LOC

### Top issues (by impact)
1. ...
2. ...
```

Ask the user via AskUserQuestion:
- Which categories to fix?
- Budget: quick cleanup / full refactoring?

### Phase 4: Execution

For each fix:
1. Read the file, understand context
2. Make the change (Edit)
3. `cargo check` — compiles?
4. If logic is affected — `cargo test --workspace`
5. Move to next

Order: simple and safe first (dead code, style), then structural.

On compilation/test failure — revert the change and move to next.

### Phase 5: Validation

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Compare with baseline: warnings removed, LOC deleted/changed.

## Rules

- Do not create abstractions "for the future" — only when duplication already exists
- Do not change public API without explicit consent
- Do not touch files you haven't read
- Do not expand refactoring beyond the agreed scope
- Atomic changes: one fix at a time, verify after each
- Output to terminal, not to report files
- Communicate in the user's language
