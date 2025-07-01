# ASIMOV Bright Data Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-brightdata-module.svg)](https://crates.io/crates/asimov-brightdata-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-brightdata-module.svg)](https://pypi.org/project/asimov-brightdata-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-brightdata-module.svg)](https://rubygems.org/gems/asimov-brightdata-module)
[![Package on NPM](https://img.shields.io/npm/v/asimov-brightdata-module.svg)](https://npmjs.com/package/asimov-brightdata-module)

[ASIMOV] module for data import powered by the [Bright Data] web data platform.

## ✨ Features

- Imports structured data from Airbnb, Amazon, Crunchbase, eBay, Facebook,
  Google, Indeed, Instagram, LinkedIn, Walmart, X (aka Twitter), Yahoo, and
  YouTube.
- Collects the raw JSON data via the Bright Data API (requires an API key).
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output in the form of JSON-LD.
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation from PyPI

```bash
pip install -U asimov-brightdata-module
```

### Installation from RubyGems

```bash
gem install asimov-brightdata-module
```

### Installation from NPM

```bash
npm install -g asimov-brightdata-module
```

### Installation from Source Code

```bash
cargo install asimov-brightdata-module
```

## 👉 Examples

```bash
export BRIGHTDATA_API_KEY="..."
```

### Fetching X Profiles

```bash
asimov-brightdata-fetcher https://x.com/bright_init   # JSON
asimov-brightdata-importer https://x.com/bright_init  # JSON-LD
```

### Fetching LinkedIn Profiles

```bash
asimov-brightdata-fetcher https://www.linkedin.com/in/orlenchner
asimov-brightdata-importer https://www.linkedin.com/in/orlenchner
asimov-brightdata-fetcher https://www.linkedin.com/company/bright-data
asimov-brightdata-importer https://www.linkedin.com/company/bright-data
```

### Fetching Crunchbase Profiles

```bash
asimov-brightdata-fetcher https://www.crunchbase.com/organization/brightdata
asimov-brightdata-importer https://www.crunchbase.com/organization/brightdata
```

### Fetching Amazon Products

```bash
asimov-brightdata-fetcher https://www.amazon.com/Master-Algorithm-Ultimate-Learning-Machine/dp/0465094279
asimov-brightdata-importer https://www.amazon.com/Master-Algorithm-Ultimate-Learning-Machine/dp/0465094279
```

## ⚙ Configuration

### Environment Variables

- `BRIGHTDATA_API_KEY`: (required) the [Bright Data API key] to use

## 📚 Reference

### Installed Binaries

- `asimov-brightdata-cataloger`: discovers entities via the Bright Data API
  _(not implemented yet)_
- `asimov-brightdata-fetcher`: collects JSON data from the Bright Data API
- `asimov-brightdata-importer`: collects and transforms JSON into JSON-LD
  _(not implemented yet)_

### Supported Datasets

| Dataset                       | URL Prefix                                   |             JSON             |             RDF              |
|:------------------------------|:---------------------------------------------|:----------------------------:|:----------------------------:|
| Airbnb                        | `https://www.airbnb.com/rooms/`              |              ✅               |              ✅               |
| Amazon                        | `https://www.amazon.com/`                    |              ✅               |              ✅               |
| &nbsp;                        | `https://www.amazon.com/sp?seller=`          |              ✅               |              ✅               |
| Crunchbase                    | `https://www.crunchbase.com/organization/`   |              ✅               |              ✅               |
| eBay                          | `https://www.ebay.com/itm/`                  |              ✅               |              ✅               |
| Facebook                      | `https://www.facebook.com/events/`           |              ✅               |              ✅               |
| &nbsp;                        | `https://www.facebook.com/groups/`           |              ✅               |              ✅               |
| &nbsp;                        | `https://www.facebook.com/marketplace/item/` |              ✅               |              ✅               |
| &nbsp;                        | `https://www.facebook.com/share/p/`          |              ✅               |              ✅               |
| Google                        | `https://www.google.com/shopping/product/`   |              ✅               |              ✅               |
| Indeed                        | `https://www.indeed.com/cmp/`                |              ✅               |              ✅               |
| Instagram                     | `https://www.instagram.com/`                 |              ✅               |              ✅               |
| &nbsp;                        | `https://www.instagram.com/p/`               |              ✅               |              ✅               |
| &nbsp;                        | `https://www.instagram.com/reel/`            |              ✅               |              ✅               |
| LinkedIn                      | `https://www.linkedin.com/company/`          |              ✅               |              ✅               |
| &nbsp;                        | `https://www.linkedin.com/in/`               |              ✅               |              ✅               |
| &nbsp;                        | `https://www.linkedin.com/jobs/`             |              ✅               |              ✅               |
| &nbsp;                        | `https://www.linkedin.com/posts/`            |              ✅               |              ✅               |
| &nbsp;                        | `https://www.linkedin.com/pulse/`            |              ✅               |              ✅               |
| Walmart                       | `https://www.walmart.com/global/seller/`     |              ✅               |              ✅               |
| &nbsp;                        | `https://www.walmart.com/ip/`                |              ✅               |              ✅               |
| X (Twitter)                   | `https://x.com/`                             |              ✅               |              ✅               |
| Yahoo                         | `https://finance.yahoo.com/quote/`           |              ✅               |              ✅               |
| YouTube                       | `https://www.youtube.com/@`                  |              ✅               |              ✅               |
| &nbsp;                        | `https://www.youtube.com/watch?v=`           |              ✅               |              ✅               |
| <img width="100" height="1"/> | <img width="550" height="1"/>                | <img width="50" height="1"/> | <img width="50" height="1"/> |

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
[JSON-LD]: https://json-ld.org
[KNOW]: https://github.com/know-ontology
[NPM]: https://npmjs.org
[Python]: https://python.org
[RDF]: https://github.com/rust-rdf
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
