---
title: "About RustSec"
---

## About RustSec

The [RustSec Advisory Database] is a repository of security advisories filed
against Rust crates published via <https://crates.io>

Advisory metadata is stored in [TOML] format (see below). The following tools
consume the data and can be used for auditing and reporing (send PRs to add yours):

* [cargo-audit]: Audit `Cargo.lock` files for crates with security vulnerabilities

[RustSec Advisory Database]: https://github.com/RustSec/advisory-db
[TOML]: https://github.com/toml-lang/toml
[cargo-audit]: https://github.com/RustSec/cargo-audit

## Reporting Vulnerabilities

To report a new vulnerability for a Rust crate, open a pull request using the
template below. See [CONTRIBUTING.md] for more information.

<a href="https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md">
  <img alt="Report Vulnerability" width="250px" height="60px" src="https://rustsec.org/assets/img/report-vuln-button.svg">
</a>

[CONTRIBUTING.md]: https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md
