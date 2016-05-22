#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/cookie")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/cookie")]
#![crate_name = "cookie"]
#![deny(missing_doc)]
#![feature(custom_derive)]
#![feature(rustc_private)]

#![allow(unused_imports)] 
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_attributes)] 
#![allow(unknown_lints)]

//! Cookie parsing/setting middleware for the [iron](https://github.com/iron/iron) framework.

extern crate time;
extern crate rustc;
extern crate regex;
extern crate url;
extern crate serde;
extern crate serde_json;
extern crate iron;
#[macro_use] extern crate hyper;
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
