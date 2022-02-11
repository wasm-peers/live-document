# Live Document

Proof of concept application showcasing the usability of [wasm-peers](https://github.com/wasm-peers/wasm-peers)
crate for easy and costless peer-2-peer WebRTC communication.

It's an application akin to Google Docs, where many users can share a document and see changes introduced by any
of them in real-time.

It was written in Rust, compiles down to WASM and can be hosted as a set of static files, no backend required.

Check out the live demo [here](http://live-document.s3-website.eu-central-1.amazonaws.com/).

## Overview

This application uses wasm-peers crate to provide many-to-many connection between each pair of connecting peers.  
Thanks to that, even if the original creator of the document leaves, copy of the work is still stored and distributed by each peer.

It also utilises [yew](https://yew.rs/) framework for creation of Single Page Application.

## Authors

Tomasz Karwowski  
[LinkedIn](https://www.linkedin.com/in/tomek-karwowski/)

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

