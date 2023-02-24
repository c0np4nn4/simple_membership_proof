# Simple membership proving system using zk-SNARK

## Description
- 부산대학교 정보보호 동아리 [KEEPER](https://keeper.or.kr/) 의 기술문서 활동 결과물입니다.

- `Cryptography`를 이용한 정보보호의 관점에서 `zk-SNARK` 를 이용한 멤버십 검증 시스템을 구현했습니다.

- 서버와 클라이언트를 구분하여 구현했으며, `Rust` 언어를 사용했습니다.

### Server
- `Hyper` 를 이용하였으며, 기본적인 구조는 아래 사이트를 참고하여 구현하였습니다.
  - [Hyper, github](https://hyper.rs)
  - [LogRocket:a minimal web service in rust using hyper](https://blog.logrocket.com/a-minimal-web-service-in-rust-using-hyper)

### Client
- 클라이언트는 `TUI`를 이용하여 프론트엔드를 만들었습니다. 
  - [tui-rs, github](https://github.com/fdehau/tui-rs)

### ZK-SNARK
- `Arkworks`를 이용하여 `zk-SNARK` 부분을 구현했습니다.
  - [arkworks, github](https://github.com/arkworks-rs)
  - [Groth16 in arkworks, github](https://github.com/arkworks-rs/groth16)
