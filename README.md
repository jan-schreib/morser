# morser
Rust crate for encoding and decoding morse code

# examples

## encoding
``` rust
extern crate morser;
use morser::Encoder;

fn main() {
    let encoder = Encoder::new();
    let plain = "sos"
    let morse = encoder.encode(plain);
}
```

##decoding

``` rust
extern crate morser;
use morser::Decoder;

fn main() {
    let decoder = Decoder::new();
    let morse = "... --- ..."
    let plain = decoder.decode(morse);
}
```
