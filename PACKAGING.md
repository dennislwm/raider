Packaging
=========

This file contains quick reminders and notes on how to package Raider.

We consider here the packaging flow of Raider version `1.0.0` for Linux:

1. **How to bump Raider version before a release:**
    1. Bump version in `Cargo.toml` to `1.0.0`
    2. Execute `cargo update` to bump `Cargo.lock`

2. **How to update Crates:**
    1. Publish package on Crates: `cargo publish`

3. **How to update Docker:**
    1. `docker build .`
    2. `docker tag [DOCKER_IMAGE_ID] valeriansaliou/raider:v1.0.0` (insert the built image identifier)
    3. `docker push valeriansaliou/raider:v1.0.0`
