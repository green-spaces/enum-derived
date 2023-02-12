use enum_derived::Rand;

#[derive(Rand, Debug)]
enum Dna {
    A,
    C,
    T,
    G,
}

fn main() {
    let mut bases = Vec::new();
    for _ in 0..4 {
        bases.push(Dna::rand());
    }
    println!("random bases: {bases:?}");
}
