# Contributing to zbus

We welcome contributions from everyone in the form of suggestions, bug reports, pull requests, and
feedback. This document gives some guidance if you are thinking of helping us.

Please reach out here in a Github issue, or in the
[#zbus:matrix.org](https://matrix.to/#/#zbus:matrix.org) Matrix room if we can do anything to help
you contribute.

## Submitting bug reports and feature requests

You can create issues [here](https://github.com/dbus2/zbus/issues/new). When
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
  more information, including the rationale.
* Please try your best to follow [these guidelines](https://wiki.gnome.org/Git/CommitMessages) for
  commit messages.
* We also prefer emoji prefixes to commit messages. Please read
  [this excellent blog post](https://gitmoji.carloscuesta.me/) for more information, including the
  rationale. However, the `gitmoji` CLI tool can be very
  [slow](https://github.com/zeenix/gimoji#rationale), so we recommend using
  [`gimoji`](https://github.com/zeenix/gimoji) instead.
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
cargo +nightly test --all-features
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

If you intend to contribute often or think that's very likely, we recommend you setup the following git
hooks:

* Pre-commit hook that goes in the `.git/hooks/pre-commit` file:

  ```sh
  if ! which rustup &> /dev/null; then
      curl https://sh.rustup.rs -sSf  | sh -s -- -y
      export PATH=$PATH:$HOME/.cargo/bin
      if ! which rustup &> /dev/null; then
          echo "Failed to install rustup"
      fi
  fi

  if ! rustup component list --toolchain nightly|grep 'rustfmt-preview.*(installed)' &> /dev/null; then
      echo "Installing nightly rustfmt.."
      rustup component add rustfmt-preview --toolchain nightly
      echo "rustfmt installed."
  fi

  echo "--Checking style--"
  cargo +nightly fmt --all -- --check
  if test $? != 0; then
      echo "--Checking style fail--"
      echo "Please fix the above issues, either manually or by running: cargo +nightly fmt --all"

      exit -1
  else
      echo "--All very stylish 😎--"
  fi
  ```

* Pre-push hook that goes in the `.git/hooks/pre-push` file:

  ```sh
  if ! which rustup &> /dev/null; then
      curl https://sh.rustup.rs -sSf  | sh -s -- -y
      export PATH=$PATH:$HOME/.cargo/bin
      if ! which rustup &> /dev/null; then
          echo "Failed to install rustup"
      fi
  fi

  if ! rustup component list --toolchain stable|grep 'clippy.*(installed)' &> /dev/null; then
      echo "Installing clippy.."
      rustup component add clippy
      echo "clippy installed."
  fi

  echo "--Analysing code 🔍--"
  cargo clippy -- -D warnings
  if test $? != 0; then
      echo "--Issues with code. See warnings/errors above--"

      exit -1
  else
      echo "--Code looks good 👍--"
  fi
  ```

## Adding public API

### Assert auto traits on items

Please make sure to add `assert_impl_all!()` macros to ensure that accidental changes to auto trait
implementations like `Send`, `Sync`, and `Unpin` can be detected easily. You should use the existing
code to see how it is done with all of the current items already. For further information see the
Rust API Guidelines on [C-SEND-SYNC].

## Conduct

In all zbus-related forums, we follow the
[Rust Code of Conduct](https://www.rust-lang.org/conduct.html). For escalation or moderation issues
please contact Zeeshan (zeeshanak@gnome.org) instead of the Rust moderation team.

[C-SEND-SYNC]: https://rust-lang.github.io/api-guidelines/interoperability.html#c-send-sync
