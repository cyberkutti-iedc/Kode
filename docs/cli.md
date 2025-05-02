# 📄 Kode CLI Commands Reference

This document provides usage details for all CLI commands available in the `kode` binary.

## 🔧 Commands

| Command               | Description                                |
|------------------------|--------------------------------------------|
| `kode run <file>`     | Runs a `.kode` or `.kdc` file              |
| `kode build <file>`   | Compiles `.kode` into `.kdc` bytecode      |
| `kode repl`           | Starts the interactive REPL                |
| `kode version`        | Prints the version number                  |
| `kode help`           | Displays CLI usage instructions            |

## ⚙️ Options

| Flag           | Description                                 |
|----------------|---------------------------------------------|
| `--verbose`    | Prints additional internal debug information|
| `--optimize`   | Enables code optimization (planned)         |
| `--no-run`     | Only compile, do not execute                |
| `--time`       | Shows execution time                        |

## Examples

```bash
kode run examples/hello.kode --verbose
kode build examples/main.kode --no-run
kode repl
