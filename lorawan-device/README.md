# lorawan-device

[![Latest Version]][crates.io]
[![Docs]][doc.rs]

This is an experimental LoRaWAN device stack with both non-blocking (`nb_device`) and async (`async_device`)
implementations. Both implementations have their respective `radio::PhyRxTx` traits that describe the radio interface
required.

Note: The `lorawan-radio` feature in the `lora-phy` crate provides `LorawanRadio` as an async implementation of
`radio::PhyRxTx`.

Both stacks share a dependency on the internal module, `mac` where LoRaWAN 1.0.x is approximately implemented:

- Class A device behavior
- Class C device behavior (async only, enabled by default with the `class-c` feature)
- Over-the-Air Activation (OTAA) and Activation by Personalization (ABP)
- CFList is supported for fixed and dynamic channel plans
- Regional support for AS923_1, AS923_2, AS923_3, AS923_4, AU915, EU868, EU433, IN865, US915 with following caveats:
  * Regional power limits are not enforced ([#168](https://github.com/lora-rs/lora-rs/issues/168))
  * FSK and LR-FHSS modulations are not supported

**Currently, not all MAC commands are fully implemented**. These commands
are gated behind the "experimental" feature.

Furthermore, both async and non-blocking implementation do not implement any retries for failed joins or failed
confirmed uplinks. It is up to the client to implement retry behavior; see the examples for more.

Please see [examples](https://github.com/lora-rs/lora-rs/tree/main/examples) for usage.

A public chat on LoRa/LoRaWAN topics using Rust is [here](https://matrix.to/#/#public-lora-wan-rs:matrix.org).

[Latest Version]: https://img.shields.io/crates/v/lorawan-device.svg
[crates.io]: https://crates.io/crates/lorawan-device
[Docs]: https://docs.rs/lorawan-device/badge.svg
[doc.rs]: https://docs.rs/lorawan-device
