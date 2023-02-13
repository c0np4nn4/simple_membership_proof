# 2022_KEEPER_tech_report

### Server-Client
- The basic structure of the server follows the source code described in [[LogRocket:a minimal web service in rust using hyper](https://blog.logrocket.com/a-minimal-web-service-in-rust-using-hyper/)].

- `Client` uses [tui](https://github.com/fdehau/tui-rs) for ui.

### ZK-SNARK
- We use [arkworks](https://github.com/arkworks-rs) for generating and verifying the proof using [Groth16](https://github.com/arkworks-rs/groth16)
