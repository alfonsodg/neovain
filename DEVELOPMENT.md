# Development

Where this fork's work lives, what each branch carries, and the rules for
merging it. Updated whenever the branch state changes.

## Branches

- **`develop`** — everything: the fixes proposed upstream plus the
  fork's own fixes. New work lands here.
  - `e9d5854` — `fix(installer)`: installers, package metadata and
    README commands resolve to this fork, with `NEOVAIN_REPO=owner/repo`
    as the documented override (#4).
  - `7d66aeb` — `feat(safe)`: the opt-in agent-safe profile,
    `--safe`/`NEOVAIN_SAFE=1` plus `--workspace DIR`/
    `NEOVAIN_WORKSPACE=DIR` (#5).
  - `f9a85f9` — `fix(safe)`: the optional `!` before the `=` register
    is normalized, so `:put! =...`, `:silent put! =...` and the
    `:g`/`:v` nests are refused again (#5, re-audit).
  - `f9a5c46` — `ci(installers)`: the installer workflow exercises
    the default source, an explicit `NEOVAIN_VERSION` pin and the
    `NEOVAIN_REPO` override, against the fork's `v0.2.0` release (#4).
- **`main`** — the six commits of
  [PR #18](https://github.com/kbrock84/neovain/pull/18) to the upstream
  (`kbrock84/neovain`): write/quit steps rejected, ex-only rejects the
  buffer leavers, the range orphan warning, and their docs and tests.
  Frozen at `289c4ae` while that pull request is open.

## Merge rules

- Do not merge `develop` into `main` while PR #18 is open. `main` is
  the pull request's source branch, and `e9d5854` (#4) points the
  installers at this fork: proposing that upstream would be wrong.
- The safe profile (`7d66aeb`, #5) could be valuable upstream on its
  own. If the maintainer wants it, cherry-pick it onto a branch from
  `main` and open a separate pull request; do not carry it through #18
  by merging `develop`.
- `main` stays untouched until PR #18 is merged or closed.
- The agent never closes an issue: the criteria evidence goes in an
  issue comment and the issue moves to `status::review`; the
  developer closes it manually.

## Open threads

- Copilot's review on PR #18: six findings, triaged and unfixed.
  `ex_word` does not consume full vim ranges or modifiers
  (`src/validate.rs`), the ex-only check misses nesting and
  abbreviations, the ordered-list detection only handles one-digit
  markers (`src/summary.rs`), plus two documentation nits (the CHANGELOG
  compare link and a README indentation). That code lives on `main`, so
  fixes start from `main` and can ride PR #18.
- `site/*.html` still links to `kbrock84/neovain` (github navigation,
  `downloadUrl`, releases and license links). It was outside #4's
  scope; decide whether the deployed fork site should install from the
  fork, then change it in a follow-up.
- The fork's first release, `v0.2.0`, was published on 2026-10-09 by
  pushing that tag to `.github/workflows/release.yml`; the tag must
  keep matching the `Cargo.toml` version or the workflow refuses it.
- Issues #1-#3 (single-writer guarantee, ex-only buffer leavers, range
  orphan warning) were reopened and moved to `status::review`: the
  agent had closed them directly, which the workflow forbids. Their
  evidence stands and their commits belong to PR #18; closing them is
  the developer's call.
- Issues #4 and #5 carry their re-audit evidence and also sit in
  `status::review`; their fixes are merged into `develop` and
  closing them is the developer's call.

## Verification

The suite is `cargo test --release` (unit, cli, ex-only, installers,
safe) plus `python3 bench/export_site.py --check`. CI runs both with
Neovim required. Neovim 0.9 or newer is the floor; the Rust MSRV is
1.74.
