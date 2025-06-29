# sumsub-webhook-cli

## Sumsub Webhook CLI

This CLI tool lets you simulate [Sumsub](https://sumsub.com) webhook events in a local development environment.
You can:

- Send predefined events (e.g. `applicantReviewed`)
- Load custom JSON payloads
- Sign requests with a secret
- Retry delivery with exponential backoff

### Requirements

To build and run this CLI tool, you need to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

Install them using [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once installed, you can run the CLI like this:

```bash
cargo run -- event --event applicant-reviewed --secret "your-secret" --url "your-webhook-host-url"
```

Or install globally with:

```bash
cargo install sumsub-webhook-cli
```

### Usage

```bash
sumsub-webhook-cli event --event applicant-reviewed --secret xxx --url http://localhost:3000/webhook
sumsub-webhook-cli payload --path ./payload.json --secret xxx --url http://localhost:3000/webhook
```

### Installation

```bash
cargo install sumsub-webhook-cli
```

### License

Licensed under either of:

- MIT license ([LICENSE-MIT](https://opensource.org/license/mit/))
- Apache License, Version 2.0 ([LICENSE-APACHE](https://www.apache.org/licenses/LICENSE-2.0))

You may choose either license.

License: MIT OR Apache-2.0
