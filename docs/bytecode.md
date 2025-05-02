
### 📁 `docs/bytecode_format.md` – Bytecode Format 

```md
# 📁 Kode Bytecode Format (.kdc)

## Overview

The `.kdc` format stores serialized Abstract Syntax Trees (ASTs) as bytecode using [`bincode`](https://github.com/bincode-org/bincode).

## Structure

- Magic header: `0x4B 0x44 0x43` (optional)
- Version info: `u8` (planned)
- Body: serialized `Vec<ast::Statement>` from `ast.rs`

## Generation

Bytecode is generated using:

```rust
let bytecode = bincode::serialize(&ast)?;
fs::write("program.kdc", bytecode)?;
````

## Execution

To run:

```bash
./kode run program.kdc
```

The runtime will deserialize and execute the AST using `Interpreter::run()`.

````
