# Megalodon
[![Crates.io](https://img.shields.io/crates/v/megalodon)](https://crates.io/crates/megalodon)
[![docs.rs](https://img.shields.io/docsrs/megalodon/latest)](https://docs.rs/megalodon/latest/megalodon/)
[![Build](https://github.com/h3poteto/megalodon-rs/actions/workflows/build.yml/badge.svg)](https://github.com/h3poteto/megalodon-rs/actions/workflows/build.yml)
[![GitHub](https://img.shields.io/github/license/h3poteto/megalodon-rs)](LICENSE.txt)

The `megalodon` is a client library for Mastodon and Pleroma. It provides REST API and streaming method which uses WebSocket. By using this library, you can take Mastodon and Pleroma with the same interface.
This library is Rust version of [megalodon](https://github.com/h3poteto/megalodon).

## Features
- [ ] REST API
  - [x] Mastodon
  - [ ] Pleroma
- [ ] Streaming with WebSocket
  - [x] Mastodon
  - [ ] Pleroma
- [ ] Proxy support


## Usage
Add your `Cargo.toml` like this:

```
[dependencies]
megalodon = { version = "0.1" }
```

### Making Mastodon request
For a request without authentication.

```rust
let client = megalodon::generator(
  megalodon::SNS::Mastodon,
  String::from("https://fedibird.com"),
  None,
  None,
);
let res = client.get_instance().await?;
println!("{:#?}", res.json());
```

### Making Mastodon request with authentication
For a request with authentication.

```rust
let client = megalodon::generator(
  megalodon::SNS::Mastodon,
  String::from("https://fedibird.com"),
  Some(String::from("your access token")),
  None,
);
let res = client.verify_account_credentials().await?;
println!("{:#?}", res.json());
```

## License
The software is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
