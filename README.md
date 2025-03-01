# Enum-Derived

Use enum-derived's Rand macro to generate random variants of your enums and structs. All fields are populated with independent random values.

Need custom constraints applied to a variant or field? Use the `#[custom_rand(your_function)]` attribute to override the default behavior or extend support to types without default support.

Need some variants to be generated more ofter? Use the `#[weight(VARIANT_WEIGHT)]` to change the distribution.

[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/green-spaces/enum-derived/actions/workflows/build.yml/badge.svg?branch=main)

---

## Rand

Rand allows for a random variant of an enum, or struct, to be generated.

The [rand] crate's [rand::random] method is used for the default implementation of [Rand]. Unsupported variants can us the `#[custom_rand(your_function)]` to extend the functionality.

### Note

Support for String and Vec<T> has been added. The default implementation will generate a String/Vec with a lenght between 1 and 64 elements.

## Example

```rust
use enum_derived::Rand;

#[derive(Rand)]
struct Weather {
    wind_speed: u8,
    #[custom_rand(rand_temp)]
    temperature: f32,
    cloudy: bool,
    location: String,
}

#[derive(Rand)]
enum TravelLogEntry {
    Airplane {
        weather: Weather,
        altitude: u16
    },
    Boat(
        Weather,
        #[custom_rand(rand_boat_speed)]
        u32,
    ),
    #[custom_rand(always_has_sunroof)]
    Car {
        has_sunroof: bool,
    },
    #[weight(3)]
    SpaceShip,
}
#[derive(Rand)]
pub struct TravelLog(Vec<TravelLogEntry>);

fn main() {
    let travel_log = TravelLog::rand();
}

# fn always_has_sunroof() -> TravelLogEntry {
#     TravelLogEntry::Car { has_sunroof: true }
# }
#
# fn rand_boat_speed() -> u32 {
#     thread_rng().gen_range(5..50)
# }
# 
# fn rand_temp() -> f32 {
#    thread_rng().gen_range(-20.0..120.0)
# }
# 
# use rand::{thread_rng, Rng};
 ```
