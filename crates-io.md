<!-- Enum-Derived readme rendered on crates.io -->

**Enum-Derived adds new functionalityto enums**

---

You may be looking for:

- [API documentation](https://docs.rs/enum-derived)

## Enum-Derived in action

```rust
use enum_derived::Rand;

#[derive(Rand, Debug)]
pub enum Dna {
    A,
    C,
    T,
    G
}

fn main() {

    let base = Dna::rand();

    println!("Random Base: ${base:?}");
}
```
