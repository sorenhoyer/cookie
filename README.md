cookie [![Build Status](https://secure.travis-ci.org/iron/cookie.png?branch=master)](https://travis-ci.org/iron/cookie)
====

> Cookie parsing and setting middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
fn main() {
    let mut server: Server = Iron::new();
    server.chain.link(CookieParser::new()));
    server.chain.link(FromFn::new(echo_cookies));
    server.listen(::std::io::net::ip::Ipv4Addr(127, 0, 0, 1), 3000);
}

fn echo_cookies(req: &mut Request, _: &mut Response) -> Status {
    let cookie = req.extensions.find::<Cookie>().unwrap();
    for (key, value) in cookie.map.iter() {
        println!("{}:\t{}", *key, *value)
    }
    Continue
}
```

## Overview

cookie is a part of Iron's [core bundle](https://github.com/iron/core).

- Set and parse cookies from the browser
- Use signed cookies (using an HMAC)
- Use JSON cookies

## Installation

If you're using a `Cargo.toml` to manage dependencies, just add cookie to the toml:

```toml
[dependencies.cookie]

git = "https://github.com/iron/cookie.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/cookie)

Along with the [online documentation](http://docs.ironframework.io/cookie),
you can build a local copy with `cargo doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
