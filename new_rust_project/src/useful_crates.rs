/*
*   Concurrency Crates
*   tokio: Automatic async thread management. Useful for non-blocking IO where most time is spent waiting aka network requests for file IO
*   mio: Mio is what Tokio is built off of. Mio is more low level with manual event handling and no async
*   rayon: Handling parallel computations. Useful for calculations/processes which can be paralleled.
*   crossbeam: Complex multi-thread additional features + cross-thread channels for comms

*   Mutex
*   parking_lot: Smaller, faster implementation of Mutex RwLock, Condvar, and Once than standard library

*   Byte Stuff
*   bytes: Allows creating byte buffers
*   base64: Encodes and decodes base64

*   Serialization
*   serde: Generic serialization/deserialization into all kinds of different forms (json, yaml, toml, url, csv)

*   Image
*   image: Handles image processing and encoding/decoding

*   Numbers
*   uint: Allows for integers larger than u64. They need to be FIXED SIZE though, so if you add two together which are larger than the allotted size, you get overflow
*   num_bigint: Allows for integers larger than u64. They can grow dynamically though, at the price of overhead
*   rand: Generate random numbers

*   Networking
*   reqwest: Simple HTTP client (send HTTP requests, not receive)

*   Crypto
*   hmac:
*   digest:
*   rsa:
*   sha1:
*   sha2:
*   uuid:

*   Plugins
*   extism: Allows all languages to be compiled into WASM and executed in any language
*/