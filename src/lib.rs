#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/cookie")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/cookie")]
#![crate_name = "cookie"]
#![deny(missing_doc)]
#![feature(phase)]
#![feature(globs)]
#![feature(custom_derive)]

//! Cookie parsing/setting middleware for the [iron](https://github.com/iron/iron) framework.

extern crate time;
extern crate rustc;
extern crate regex;
extern crate url;
extern crate serialize;
extern crate iron;
extern crate hyper;
extern crate crypto;
#[cfg(test)]
extern crate iron_test as test;

pub use cookie::Cookie;
pub use parser::CookieParser;
pub use response::SetCookie;
pub use response::HeaderCollection;

mod parser;
mod response;
mod cookie;
