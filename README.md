Voodoo - A high-performance Vulkan® API for Rust
================================================

#### [Documentation](http://docs.rs/voodoo) | [Change Log](https://github.com/cogciprocate/voodoo/blob/master/RELEASES.md)

[![](http://meritbadge.herokuapp.com/voodoo)](https://crates.io/crates/voodoo) [![](https://docs.rs/voodoo/badge.svg)](https://docs.rs/voodoo)
[![Supported platforms](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux%20%7C%20bsd-orange.svg)](https://en.wikipedia.org/wiki/Cross-platform)

### Features:

* An intuitive and idiomatic interface
* Zero additional overhead
  * No unnecessary allocations
  * No intermediate structs or extra copying
  * Builders compile to direct assignment
* Thread-safe allocation / destruction
  * Safety escape hatches available everywhere
* A minimum of boilerplate
* Non-opinionated and nothing hidden
* Complete API coverage

### Getting Started

Ensure that Vulkan drivers are installed for your device. Add the following to
your project's Cargo.toml:

```toml
[dependencies]
voodoo = "0.3.1"
```

And add the following to your crate root (lib.rs or main.rs):
```rust
extern crate voodoo;
```

### Example

Create an instance:

```rust
extern crate voodoo;

use voodoo::{Result as VdResult, Instance, ApplicationInfo, Loader};
use std::ffi::CString;

/// Initializes and returns a new loader and instance with all available
/// extension function pointers loaded.
fn init_instance() -> VdResult<Instance> {
    let app_name = CString::new("Hello!")?;

    let app_info = ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version((1, 0, 0))
        .api_version((1, 0, 0))
        .build();

    let loader = Loader::new()?;

    Instance::builder()
        .application_info(&app_info)
        .enabled_extensions(&loader.enumerate_instance_extension_properties()?)
        .build(loader)
}

fn main() {
    let _instance = init_instance().unwrap();
}

```

See [`hello.rs`] for a complete, working example adapted from
[https://vulkan-tutorial.com/](https://vulkan-tutorial.com/).

To run the example, clone this repo
(`git clone https://github.com/cogciprocate/voodoo.git`, `cd voodoo`,
then `cargo run --example hello --release`.

[`hello.rs`] screenshot:

![hello_example_screenshot.jpg][hello_example_screenshot]

### Status

* API coverage:
  * Core: 100%
  * Extensions: 70%
* Documentation: 30%
* Stability: 97%

#### Other Vulkan libraries in Rust

For a higher level, more opinionated, easier to use Vulkan API that does more
for you see the [Vulkano] project.

Other low-ish level Rust libraries include [dacite] and [ash]. See [vks] for a
direct ffi interface.

[`hello.rs`]: https://github.com/cogciprocate/voodoo/blob/master/examples/hello.rs
[hello_example_screenshot]: https://github.com/cogciprocate/voodoo/blob/master/examples/hello_example_screenshot.jpg
[Vulkano]: https://github.com/vulkano-rs/vulkano
[dacite]: https://gitlab.com/dennis-hamester/dacite/tree/master/dacite
[vks]: https://gitlab.com/dennis-hamester/vks
[ash]: https://github.com/MaikKlein/ash

<br/>*“Vulkan and the Vulkan logo are registered trademarks of the Khronos Group Inc.”*