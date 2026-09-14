# 🎲 Kiouba

![License](https://img.shields.io/github/license/OmarPGH/Kiouba?color=green)
![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)

## Architecture Overview
Kiouba follows a simple modular structure:

| Module | Responsibility |
| :--- | :--- |
| `generate_name.rs` | Generates random names using configurable length constraints. |
| `input.rs` | Collects and validates user preferences through interactive prompts. |
| `main.rs` | Coordinates the application flow and displays generated results. |

---
## 🛠️ Requirements

Before running Kiouba, make sure you have:
- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## 📦 Installation

Clone the repository:
```bash
git clone https://github.com/OmarPGH/Kiouba.git
```

Navigate to the project directory:
```bash
cd Kiouba
```

Build the project:
```bash
cargo build
```

## 🚀 Usage

​Run Kiouba using Cargo:
```bash
cargo run
```

The application will guide you through the configuration process.
### Example Session
```text
number of names you need [5]:
min number of letters [4]:
max number of letters [7]:

Result [1]: kxqwe - abcds - rtyui - plmno - zxcvb

Do you want another one? [Y/n]:
```

You can continue generating new batches or exit the application when finished.

## ​🧪 Testing

​Run the test suite:
```bash
cargo test
```

Kiouba includes unit tests covering the core name generation functionality.

## ​📚 Dependencies

| Dependency | Purpose |
| :--- | :--- |
| [`rand`](https://crates.io/crates/rand) | Generates random characters and lengths. |
| [`dialoguer`](https://crates.io/crates/dialoguer) | Provides interactive terminal prompts and confirmations. |

## ⚖️ License

This project is licensed under the **Apache-2.0**. See the LICENSE file for details.

## 👨‍💻 Author

**Omar Gamal** - Creator and Maintainer
