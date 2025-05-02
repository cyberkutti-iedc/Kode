# 🤝 Contributing to Kode

First off, thank you for considering contributing to **Kode** — your help makes this language better for everyone!

Kode is a fast, interpreted programming language written in Rust. Whether you're fixing bugs, improving documentation, or adding new features, you're welcome here.

## 🧰 Project Setup

To get started:

1. **Fork** the repository
2. **Clone** your fork

```bash
git clone https://github.com/cyberkutti-iedc/kode
cd kode
```

3. **Build the project**

```bash
cargo build
```

4. **Run the CLI**

```bash
./target/debug/kode run examples/hello.kode
```

## 🧪 Running Tests

Run the test suite with:

```bash
cargo test
```

If you're adding new features, please include appropriate tests in the `tests/` directory.

## 💡 Submitting a Pull Request

When you're ready:

1. Create a new branch for your change:

```bash
git checkout -b feature/my-new-feature
```

2. Make your changes and commit them:

```bash
git add .
git commit -m "Add feature: my new feature"
```

3. Push to your fork:

```bash
git push origin feature/my-new-feature
```

4. Open a pull request on GitHub.

## 🧼 Code Style

* Use **Rust idioms** and follow the [Rust style guide](https://doc.rust-lang.org/1.0.0/style/style/index.html)
* Format code with `cargo fmt`
* Lint with `cargo clippy` if possible
* Use `snake_case` for function and variable names
* Write descriptive comments for non-obvious code
* Keep functions small and focused on a single task

## 🧭 Where to Contribute

* **Bug fixes**: Check the [issues](https://github.com/sreerajvr/kode/issues) for reported bugs
* **Documentation**: Improve README, docs, or inline code comments
* **Examples**: Add example programs to showcase language features
* **Language features**: Implement items from the [roadmap](docs/roadmap.md)
* **Standard library**: Help build the standard function library
* **Performance**: Optimize the interpreter or bytecode execution
* **Testing**: Add more unit and integration tests

## 📜 Licensing

By submitting your code, you agree to license your contribution under the [MIT License](LICENSE).

## 🙋 Need Help?

Open an issue with the `question` label or reach out to the maintainers for guidance.

---

Thanks again for your interest in contributing to Kode! 🎉

*Document maintained by Sreeraj V Rajesh*