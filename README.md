# NewProject
This is a simple starting repo for making a new project in Rust with benchmarks. Most of your code will go into `lib/`. Your `main` function will live in `bin/`. This also includes benchmarks using [Criterion](https://github.com/bheisler/criterion.rs). Just use `cargo bench` to run the benchmarks in `lib/benches/`.

# Using
You can start using this template in one of two ways:
```
# Remember to fill in the template values in Cargo.toml
$ git clone https://github.com/adamchalmers/new_rust_project

# Or let cargo-generate fill them in for you
$ cargo generate --git https://github.com/adamchalmers/new-rust-project
```

# Why?
Criterion can only benchmark code from libraries. But I always want to benchmark my binaries. So I wrote this nice directory structure which lets you keep most of your code in `lib/` (so it can be benchmarked). And then it includes a binary in a different package. This has the nice side-effect of letting you separate your dependencies. For example, your library can use async/await without pulling in any executor or runtime. Then, your binary can import a heavy dependency like `tokio` to actually execute the async functions. This reduces compile times for users of your library :)
