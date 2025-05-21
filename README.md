# ASIMOV Bright Data Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-brightdata-module)](https://crates.io/crates/asimov-brightdata-module)

[ASIMOV] module for data import powered by the [Bright Data] web data platform.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-brightdata-module
```

## 👉 Examples

```console
$ export BRIGHTDATA_API_KEY="..."

$ asimov-brightdata-importer https://www.linkedin.com/in/arto/
```

## ⚙ Configuration

### Environment Variables

- `BRIGHTDATA_API_KEY`: (required) the [Bright Data API key] to use

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-brightdata-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-brightdata-module&text=asimov-brightdata-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-brightdata-module&title=asimov-brightdata-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-brightdata-module&t=asimov-brightdata-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-brightdata-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-brightdata-module)

[ASIMOV]: https://github.com/asimov-platform
[Bright Data]: https://brightdata.com/products/web-scraper
[Bright Data API key]: https://docs.brightdata.com/general/account/api-token
