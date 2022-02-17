Continuity -- a true hybrid KVM.
==================================

## About Continuity

This is a new, experimental hybrid (software & hardware) KVM, written in Rust.

The idea is for a software component, running on low-power hardware, such as a
Raspberry Pi. I'm also exploring the possibility of a Steam Link being able
to run the software as well.

### History of KVM switches

There are several components of the Continuity suite, but first, I should
explain *what* a KVM switch is. A KVM switch is not related to the
virtualization technology, but instead, it stands for 'keyboard-video-mouse'.

Traditionally, a KVM switch would be purely hardware. The idea is to have a
single set of input devices (mouse, keyboard), and an output device,
controlled and outputted to by one computer at a time, from a selection of 2+
machines connected to the switch.

Then, software KVM switches appeared, such as CosmoSynergy, Synergy, Barrier,
and more
recently, Input Leap. (disclaimer: Barrier I used to work on, and Input Leap is
the fork I
now help maintain). There are other types of software KVM switches.

### About Continuity (cont.)

Continuing on, the components that Continuity suite encompasses software-wise
are:

- The 'Host' daemon
- The Client 'adapter'
- The USB via IP driver

#### Host daemon

The Host daemon is what runs on the low-powered or other primary input/output
device. It's a full-screen SDL application running on the [`cage`][cage]
Wayland compositor. As well as the
graphical side of things, the Host daemon also acts as the primary component
of the Continuity suite. It processes events and I/O from the connected USB
devices (more on that later on), and allows for the user to change the
selected computer via keyboard shortcuts, or serial/USB(?)/GPIO peripherals.

#### Client adapter

Combining the Host daemon with the second component: the Client 'adapter'.
This component is for the Host daemon to establish a bidirectional connection
with. The Client adapter is responsible for sharing the screen of the machine
with the Host daemon.

When the computer the Client adapter is running on is
selected by the user, it will begin broadcasting the screen (via
platform-specific APIs) to the Host daemon over the network, within an
encrypted UDP stream - the UDP protocol will allow for smooth streaming.

#### USB via IP driver

The USB via IP driver is contained in [this][usbvip] repository. There is
already a USB via IP driver, but it doesn't appear to be encrypted, or
currently work on newer versions of Windows, according to one bug report.

This driver has yet to be begun, but the protocol will be msgpack, encrypted
with TLS over TCP.

## How do I install this software?

Unfortunately, it's nowhere near ready yet.

I will be soon be creating a contribution guide, ways to get started, issues
to be tackled, etc.

## How can I donate?

Donations aren't setup yet, but check back later! I plan to set up a Pateron
at some point.

## License

I believe that a project of this calibre deserves to remain freely available,
and also hackable - to that end, I am licensing it under the [GPLv2 license]
[license]

[cage]: https://github.com/Hjdskes/cage
[usbvip]: https://github.com/continuity-kvm/usbvip
[license]: /LICENSE
