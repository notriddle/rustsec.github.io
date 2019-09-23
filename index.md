---
title: "About RustSec"
---

The [RustSec Advisory Database] is a repository of security advisories filed
against Rust crates published via <https://crates.io> maintained by the
[Rust Secure Code Working Group].

[gitter-image]: https://badges.gitter.im/badge.svg
[gitter-link]: https://gitter.im/RustSec/Lobby

## RustSec Tools

### Command-line Utilities

- [cargo-audit]: Audit `Cargo.lock` files for crates with security vulnerabilities

### Libraries

- [`rustsec` crate]: Client library for the RustSec Advisory Database
- [`cargo-lock` crate]: Parser for `Cargo.lock` files used by RustSec

### Libraries

## Reporting Vulnerabilities

To report a new vulnerability for a Rust crate, open a pull request
against the [RustSec Advisory Database].

<a href="https://github.com/RustSec/advisory-db/blob/master/CONTRIBUTING.md">
  <img alt="Report Vulnerability" width="250px" height="60px" src="https://rustsec.org/assets/img/report-vuln-button.svg">
</a>

[RustSec Advisory Database]: https://github.com/RustSec/advisory-db
[Rust Secure Code Working Group]: https://github.com/rust-secure-code/wg
[cargo-audit]: https://github.com/RustSec/cargo-audit
[`rustsec` crate]: https://github.com/RustSec/rustsec-crate
[`cargo-lock` crate]: https://github.com/RustSec/cargo-lock
