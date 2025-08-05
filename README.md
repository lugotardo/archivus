# 📁 Archivus

**Archivus** is a high-performance and developer-friendly Rust library for local file and directory management.  
It simplifies common filesystem operations while providing powerful tools for indexing, validation, and search.

## ✨ Features

- 🔍 Fast and flexible **file search** (by name, content, extension, metadata)
- 📂 Easy **directory traversal** with filters and depth control
- 📑 Access to rich **file metadata**
- ✅ File and path **validation utilities**
- 📋 Custom **file listing** with sorting and grouping options
- ⚡ Modular design with **focus on performance and safety**
- 📚 Clean and well-documented API

---

## 📦 Installation

Add Archivus to your `Cargo.toml`:

```toml
[dependencies]
archivus = "0.1"
````

---

## 🚀 Quick Start

```rust
use archivus::prelude::*;

fn main() -> archivus::Result<()> {
    let files = FileFinder::new("./some-folder")
        .with_extension("rs")
        .recursive(true)
        .find()?;

    for file in files {
        println!("Found Rust file: {}", file.path().display());
    }

    Ok(())
}
```

---


## 🤝 Contributing

We welcome contributions, suggestions, and bug reports!
Feel free to open issues or pull requests.

---

## ⚖️ License

This project is licensed under the MIT License.
See the [LICENSE](./LICENSE) file for details.

---

## 🛠 Built with ❤️ in Rust

Archivus is part of a set of tools aimed at empowering developers with safe and performant file-handling utilities in Rust.
