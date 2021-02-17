# rustc_bug issue [#82239](https://github.com/rust-lang/rust/issues/82239)
Documentation repository of a bug in rustc

```rust
struct Field (usize);

struct Crasherini {
    field: Field,
}

impl Crasherini {
    pub fn write_to_immut_ref(&mut self){
        for value in vec![&self.field] {
            value.0 = 10;
        } 
    }
}
```

This code crash the rustc compiler using:
```
rustc 1.52.0-nightly (5fa22fe6f 2021-02-14)
binary: rustc
commit-hash: 5fa22fe6f821ac3801d05f624b123dda25fde32c
commit-date: 2021-02-14
host: x86_64-unknown-linux-gnu
release: 1.52.0-nightly
LLVM version: 11.0.1
```
With error:
```
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:306:18: impossible case reached

thread 'rustc' panicked at 'Box<Any>', /rustc/5fa22fe6f821ac3801d05f624b123dda25fde32c/library/std/src/panic.rs:59:5
```
