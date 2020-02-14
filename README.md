# libppm

Collaborators: COUSIN Cecile, DELAUNAY Clement, DEPRAZ Clement, GIANOTTI Mathias

## Usage

```rust
extern crate ppm;
use std::path::Path;
use ppm::{new_with_file, flip, invert, greyscale, save};

fn main() {
    let path = Path::new("./path_to_input_image.ppm");
    let mut image = ppm::new_with_file(path);
    image = flip(&mut image);
    image = invert(&mut image);
    image = greyscale(&mut image);
    image = save(&mut image);
    println!("{}", image);
    let output_path = Path::new("./path_to_output_image.ppm");
    image.save(output_path).unwrap();
}
```

## Documentation

This project includes documentation. To see it, launch from root directory:
```shell
cargo doc --open
```

## Tests and benchmarks

This project includes some unit tests and benchmarks. To run them, use respectively from root directory:
```shell
cargo test
cargo bench
```

## Code practices and bonus features

Best practices:
- Project architecture: https://learning-rust.github.io/docs/a4.cargo,crates_and_basic_project_structure.html#Project-Structure
- Documentation
- Unit tests
- Benchmarks
- Examples

Bonuses:
- Add new image methods: downscale and flip
- Add colors in terminal when displaying objects
- Add several methods to compute grayscale
