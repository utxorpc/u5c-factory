---
name: start-pr-queue
description: Detect uncommitted changes across the submodules, open one PR per changed submodule (committing via the git-commit convention), then present a summary and a recommended merge sequence. Use after a cross-cutting change (e.g. a bump-sdk-spec run) has left edits in multiple submodule working trees.
---

# Start PR queue

Turns scattered submodule working-tree edits into one reviewable PR per
submodule, then hands back an ordered merge plan. Built to follow a
cross-cutting run such as `bump-sdk-spec`, where several `sdks/*` (and possibly
`spec/`) have uncommitted changes that must land upstream individually.

## When to use

- One or more submodules have uncommitted working-tree changes that belong
  upstream as separate PRs.
- **Not** for the umbrella's own files — those are committed in `u5c-factory`
  directly via the `git-commit` skill, not PR'd here.

## Prerequisites

- `gh` authenticated (`gh auth status`) with push rights to the `utxorpc`
  repos; submodules use `https://github.com/utxorpc/...` remotes.
- Each changed submodule's edits are intended and complete (ideally already
  build-verified; if not, say so in the PR body).
- Clean separation: know which paths in each submodule are part of this change.

## Steps

1. **Detect.** For every submodule path in `.gitmodules`, run
   `git -C <path> status --porcelain`. Collect the ones with changes; skip
   clean ones. Report the detected set before doing anything that writes.
2. **Confirm with the user.** Opening PRs is outward-facing and hard to
   reverse. Show the list of submodules + a one-line change summary each and
   get an explicit go-ahead before pushing anything.
3. **Per changed submodule** (work inside `<path>`):
   a. Create a topic branch off the submodule's default branch:
      `git -C <path> checkout -b <type>/<short-slug>` (type matches the
      Conventional Commit type, e.g. `feat/spec-v0.19`).
   b. Stage the intended paths explicitly (never blanket `git add -A` if it
      would sweep unrelated files).
   c. Commit using the **`git-commit` skill** convention — Conventional
      Commits, **no `Co-Authored-By` trailer** (see
      `skills/git-commit/SKILL.md`; it applies to submodules too).
   d. Push the branch: `git -C <path> push -u origin <branch>`.
   e. Open the PR: `gh pr create -R utxorpc/<submodule-repo> --base <default>
      --head <branch> --title "<conventional-commit subject>" --body "<why +
      verification status + link back to the umbrella change>"`.
   f. Capture the returned PR URL/number.
4. **Do not** move umbrella submodule pointers yet — pointers are bumped only
   after the submodule PRs merge (a separate, deliberate umbrella commit).
5. **Summarize.** Present the queue and merge sequence (below).

## Merge sequence (dependency order)

SDKs consume generated code from `spec`. Order the queue so dependencies merge
first:

1. **`spec/`** PR (if present) — protocol/source of truth. Merge + publish
   its codegen artifacts first.
2. **`sdks/*`** PRs — independent of each other; may merge in parallel once
   their proto-gen dependency (from step 1) is published. A spec-bump SDK PR
   must not merge before the matching codegen package is on its registry.
3. **Umbrella pointer bump** — after submodule PRs merge, a separate
   `u5c-factory` commit moves the submodule pointers and refreshes
   `reference/sdk-parity.md`. Out of scope for this skill; note it as the
   final step.

## Output / summary format

Present a table: submodule · branch · commit subject · PR URL · verified?
(build-checked vs. unverified) · blockers. Then an explicit ordered merge
list with the rationale (spec → SDKs → umbrella pointer bump), flagging any
PR that is blocked (e.g. registry package not yet published, build unverified
due to missing toolchain).

## Verification

- Every changed submodule has exactly one branch + PR; no submodule with
  changes was skipped, no clean submodule got an empty PR.
- Each commit message is Conventional-Commits-shaped with no co-author
  trailer (`git -C <path> log -1 --format=%B`).
- The summary lists every PR URL and an unambiguous merge order.
- No umbrella submodule pointers were moved by this skill.

## Notes

- Outward-facing: branches are pushed and PRs opened on GitHub. Always
  confirm the detected set with the user first (step 2); never auto-open.
- Reuses, not duplicates, the commit convention — defer to
  `skills/git-commit/SKILL.md` for message format.
- One PR per submodule by design; do not bundle unrelated submodule changes
  into a single PR.
- If a submodule's changes are not build-verified (e.g. toolchain absent
  during a `bump-sdk-spec` run), state that explicitly in the PR body and in
  the summary so reviewers/CI cover it.
