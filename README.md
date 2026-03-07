# Komiteo

**Komiteo** is a CLI tool written in **Rust** that automatically generates Git commit messages.

The goal of the project is simple:
run `komiteo`, and the tool will analyze the changes in your repository and generate a clear, structured commit message.

No more wasting time thinking about commit messages.

---

## ✨ Features

- Automatic commit message generation
- Analyzes repository changes using `git diff`
- Clean and readable commit format
- Fast and lightweight (powered by Rust)
- Simple developer workflow

---

## ⚙️ Installation

### From source

```bash
git clone https://github.com/97mams/komiteo
cd komiteo
cargo build --release
```

The binary will be available in:

```bash
target/release/komiteo
```

You can move it to your `$PATH`:

```bash
cp target/release/komiteo /usr/local/bin
```

---

## 🚀 Usage

Inside a Git project:

```bash
komiteo
```

Komiteo will:

1. Analyze the repository changes
2. Read the `git diff`
3. Generate a structured commit message
4. Let you review the message before committing

---

## 🧠 Commit Format

Komiteo follows a common commit style:

```
<type>: <short description>
```

Example:

```
feat: add audio processing service
fix: resolve CLI argument parsing issue
refactor: simplify commit generation logic
```

---

## 🦀 Built With

- Rust
- Git CLI
- Simple CLI interface

---

## 📌 Project Goals

Komiteo aims to:

- simplify the commit workflow
- encourage better commit messages
- provide a fast CLI experience
- integrate easily with developer tools

---

## 🤝 Contributing

Contributions are welcome.

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Open a pull request

---

## 📄 License

MIT License
