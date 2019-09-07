# numbers_to_english
Wondering how to spell out "983912332312" properly?

Look no further! This simple program written in [Rust](https://www.rust-lang.org) will do just that. No fuss, just type in your number, and it will print out its english version!

No external crate, no dependencies, a vanilla program that will simply fulfill its duty!

# Spinnin' it up!
Make sure you have [Cargo installed!](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
git clone https://github.com/pscott/numbers_to_english
cd numbers_to_english
cargo run
```

# Pimpin' it up!
Tired of seing regular spaces, commas, and hyphens? Pimp your own display by passing in options.

### Examples:

Option | Example Command | Number | Result
--- | --- | --- | --- |
`--hypen` | `cargo run -- --hyphen '#'` | 23 | twenty#three
`--group_separator` | `cargo run -- --group_separator '!'` | 1234 | one thousand!thirty-four
`--spacing` | `cargo run -- --spacing '?'` | 8000 | eight?thousand

Of course, you're free to mix them up (be careful though, the next argument after each option is taken literally!).
