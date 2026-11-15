- Changelog fragments (SPEC.md §9.4). A pull request records its change as a
  new file `changes/<slug>.md` and no longer edits CHANGELOG.md, so a landing
  no longer makes every open pull request conflict on the `## Unreleased`
  list and restart its gate. `cargo xtask bump-version` folds the
  `## Unreleased` entries and then every fragment, in file-name order, under
  `## X.Y.Z`, leaves an empty `## Unreleased`, and deletes the fragments, all
  or nothing. `cargo xtask validate-model` refuses a fragment that is empty,
  holds a `#` or `##` heading, or is not a `.md` file, and the deferral audit
  scans the fragments as it scans CHANGELOG.md.
