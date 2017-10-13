Version       (UNRELEASED)
==========================

New Features
------------

* Debug report printing can now be optionally enabled during instance creation
  by passing `true` to the `InstanceBuilder::print_debug_report` method.
  Enabling this automatically loads the necessary extensions and wires up a
  callback function which simply prints the messages to stdout.


Breaking Changes
----------------

* The process of creating and accessing device queues has been redesigned. See
  the [`hello.rs`] example for usage.


[`hello.rs`]: https://github.com/cogciprocate/voodoo/blob/master/examples/hello.rs


Version 0.1.0 (2017-10-12)
==========================

Features
--------

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
* Useful documentation

Getting Started
---------------

Ensure that Vulkan drivers are installed for your device. Add the following to
your project's Cargo.toml:

```toml
[dependencies]
voodoo = "0.1"
```

And add the following to your crate root (lib.rs or main.rs):
```rust
extern crate voodoo;
```


Example
-------

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


Status
------

* API coverage:
  * Core: 100%
  * Extensions: 70%
* Documentation: 5%
* Stability: 90%