# Contributing to zbus

We welcome contributions from everyone in the form of suggestions, bug reports, pull requests, and
feedback. This document gives some guidance if you are thinking of helping us.

Please reach out here in a Github issue, or in the
[#zbus:matrix.org](https://matrix.to/#/#zbus:matrix.org) Matrix room if we can do anything to help
you contribute.

## Submitting bug reports and feature requests

You can create issues [here](https://github.com/z-galaxy/zbus/issues/new). When
reporting a bug or asking for help, please include enough details so that the people helping you
can reproduce the behavior you are seeing. For some tips on how to approach this, read about how to
produce a [Minimal, Complete, and Verifiable Example](https://stackoverflow.com/help/mcve).

When making a feature request, please make it clear what problem you intend to solve with the
feature, any ideas for how the crate in question could support solving that problem, any possible
alternatives, and any disadvantages.

## Submitting Pull Requests

Same rules apply here as for bug reports and feature requests. Plus:

* We prefer atomic commits. Please read
  [this excellent blog post](https://www.aleksandrhovhannisyan.com/blog/atomic-git-commits/) for
  more information, including the rationale. For larger changes addressing several packages
  consider splitting your pull request, using a single commit for each package changed.
* Please try your best to follow
  [these guidelines](https://handbook.gnome.org/development/commit-messages.html) for commit
  messages.
* We also prefer adding [emoji prefixes to commit messages](https://gitmoji.carloscuesta.me/). Since
  the `gitmoji` CLI tool can be very [slow](https://github.com/zeenix/gimoji#rationale), we
  recommend using [`gimoji`](https://github.com/zeenix/gimoji) instead. You can also pick an emoji
  direcitly from [here](https://zeenix.github.io/gimoji/) (please remember to copy the emoji itself
  and not the `:emoji-code:` string, by just clicking on it). **NOTE:** This is a curated list of
  emojis that have specific meanings. Please use one of the methods recommended here to select/fetch
  the most appropriate one. 🙏
* Add a prefix indicating the packages being changed. Use either the package name or an abbreviation
  (for example, `zb` for `zbus`, `zv` for `zvariant` etc). If a commit touches multiple packages,
  separate with a comma. For example, for a commit changing the packages `zbus` and `zvariant`,
  prefix the commit message with `zb,zv: `.
* Add details to each commit about the changes it contains. PR description is for summarizing the
  overall changes in the PR, while commit logs are for describing the specific changes of the
  commit in question.
* When addressesing review comments, fix the existing commits in the PR (rather than adding
  additional commits) and force push (as in `git push -f`) to your branch. You may find
  [`git-absorb`](https://github.com/tummychow/git-absorb) and
  [`git-revise`](https://github.com/mystor/git-revise) extremely useful, especially if you're not
  very familiar with interactive rebasing and modifying commits in git.

### Legal Notice

When contributing to this project, you **implicitly** declare that:

* you have authored 100% of the content,
* you have the necessary rights to the content, and
* you agree to providing the content under the [project's license](LICENSE).

## Running the test suite

We encourage you to check that the test suite passes locally before submitting a pull request with
your changes. If anything does not pass, typically it will be easier to iterate and fix it locally
than waiting for the CI servers to run tests for you.

```sh
# Run the full test suite, including doc test and compile-tests
cargo test --all-features
```

Also please ensure that code is formatted correctly by running:

```sh
cargo +nightly fmt --all
```

and clippy doesn't see anything wrong with the code:

```sh
cargo clippy -- -D warnings
```

Please note that there are times when clippy is wrong and you know what you are doing. In such
cases, it's acceptable to tell clippy to
[ignore the specific error or warning in the code](https://github.com/rust-lang/rust-clippy#allowingdenying-lints).

If you intend to contribute often or think that's very likely, we recommend you setup the git hook
scripts contained within this repository. You can enable them with:

```sh
cp .githooks/* .git/hooks/
```

## Conduct

In all zbus-related forums, we follow the
[Rust Code of Conduct](https://www.rust-lang.org/conduct.html). For escalation or moderation issues
please contact Zeeshan (zeeshanak@gnome.org) instead of the Rust moderation team.

