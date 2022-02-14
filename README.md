Continuity -- a true hybrid KVM.
==================================

## About

This is a new, experimental hybrid (software & hardware) KVM, written in Rust.

The idea is for a software component, running on low-power hardware, such as a
Raspberry Pi (minimum requirements apply).

Continuity acts as a server that is connected to various USB devices that are to
be shared, including a mouse and keyboard. With Continuity, each 'client' PC is
connected to the server running on say, the Raspberry Pi, and shares their
screen to the server.

Continuity's host will display the currently connected client, and allow for
peripherals to be shared. It will use technology like VNC or WebRTC (TBD) to
stream the screen. For obvious reasons, this solution will work best on wired,
LAN-based environments and all communications will be encrypted by default.

The Continuity host can be extended to multiple monitors, and this will be
reflected on the client software which would be a part of the Continuity suite.

Continuity will always remain freely available, under the terms of the GPLv2
license.

## Context

Continuity originates from my (@shymega) experiences with hardware KVMs, and the cost of
dual-monitor and other KVM switches available.

## License

This project is [licensed][license] under the GPLv2.

[license]: /LICENSE
