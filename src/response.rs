//! Setting functionality - set cookie data

use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::DEFAULT_ENCODE_SET;

//use serialize::json::{Json, Number, String, Boolean, List, Object, Null};
use serde_json;
use iron::Response;
use super::Cookie;
use time::Tm;
use std::collections::BTreeMap;

use iron::headers::{Header, HeaderFormat};
use iron::error::HttpError;
use std::fmt::{Formatter, Error};


use hyper::header::Headers;
header! { (SetCookieRequestHeader, "Set-Cookie") => [String] }

impl Header for SetCookieRequestHeader {
    fn header_name() -> &'static str {
        &"SetCookieRequestHeader"
    }

    fn parse_header(raw: &[Vec<u8>]) -> Result<Self, HttpError>{
        Ok(SetCookieRequestHeader("test".to_string()))
    }
}

impl HeaderFormat for SetCookieRequestHeader {
    fn fmt_header(&self, f: &mut Formatter) -> Result<(), Error>{
        Ok(())
    }
}

/// Set cookies.
///
/// This trait is added as a mix-in to `Response`, allowing
/// simple cookie-setting.
pub trait SetCookie {
    /// Set a cookie.
    ///
    /// Set cookies directly on the response with `res.set_cookie("coo=kie;")`.
    /// Only one cookie may sent per response, with the given key/value.
    /// Doing otherwise will result in ***undefined behavior***.
    ///
    /// Keys/values may contain restricted characters, but they will be URI encoded in the cookie.
    ///
    /// They will be decoded when the cookie is returned to the server.
    ///
    /// Cookies ***must*** be set before the response body is sent.
    /// Headers are flushed as soon anything is sent in the response body.
    fn set_cookie(&mut self, &Cookie, (String, String), HeaderCollection);

    /// Set a cookie as JSON.
    ///
    /// Cookies set as JSON will be available under `cookie.json`.
    /// Otherwise, they behave exactly as normally serialized cookies.
    ///
    /// Note that restricted characters will still be URI encoded in your cookie.
    ///
    /// They will be decoded when the cookie is returned to the server.
    fn set_json_cookie(&mut self, &Cookie, (String, BTreeMap<String, serde_json::Value>), HeaderCollection);
}

impl SetCookie for Response {
    fn set_cookie(&mut self,
                  signer: &Cookie,
                  (key, value): (String, String),
                  options: HeaderCollection) {
        //self.headers.extensions.insert("Set-Cookie".to_String(),
        self.headers.set(SetCookieRequestHeader(
            "test".to_string()
            // match signer.sign(&value) {
            //     Some(signature) => {
            //         utf8_percent_encode(key.as_slice(), DEFAULT_ENCODE_SET )
            //             .append("=")
            //             .append("s:")
            //             .append(utf8_percent_encode(value.as_slice(), DEFAULT_ENCODE_SET ).as_slice())
            //             .append(".")
            //             .append(signature.as_slice())
            //     },
            //     None            => {
            //         utf8_percent_encode(key.as_slice(), DEFAULT_ENCODE_SET )
            //             .append("=")
            //             .append(utf8_percent_encode(value.as_slice(), DEFAULT_ENCODE_SET ).as_slice())
            //     }
            // }.append(options.to_cookie_av().as_slice())
        ));
    }

    fn set_json_cookie(&mut self,
                       signer: &Cookie,
                       (key, value): (String, BTreeMap<String, serde_json::Value>),
                       options: HeaderCollection) {
        //let json = "j:".to_String().append(Stringify_json(&value).as_slice());
        let json = "{}".to_string(); //"j:".to_String().append(serde_json::to_string(&value).unwrap()/*.as_slice()*/);
        
        self.set_cookie(signer, (key, json), options)
    }
}

// fn Stringify_json(json: &Json) -> String {
//     match *json {
//         Object(ref object) => {
//             let obj: Vec<String> = object.iter().map(Stringify_pair).collect();
//             "{".to_String().append(obj.connect(",").as_slice()).append("}")
//         },
//         List(ref list)     => {
//             let ary: Vec<String> = list.iter().map(Stringify_json).collect();
//             "[".to_String().append(ary.connect(",").as_slice()).append("]")
//         },
//         Number(number) => number.to_String(),
//         String(ref String) => "\"".to_String().append(String.as_slice()).append("\""),
//         Boolean(true)      => "true".to_String(),
//         Boolean(false)     => "false".to_String(),
//         Null               => "null".to_String()
//     }
// }

// fn Stringify_pair((key, val): (&String, &Json)) -> String {
//     "\"".to_String().append(key.as_slice()).append("\":").append(Stringify_json(val).as_slice())
// }

/// The headers used to set a cookie.
///
/// These headers are defined by [RFC 6265](http://tools.ietf.org/html/rfc6265)
pub struct HeaderCollection {
    /// An absolute date/time at which this cookie should expire.
    pub expires:    Option<Tm>,
    /// A relative time (in seconds) at which this cookie should expire.
    pub max_age:    Option<u32>,
    /// The scope of the cookie.
    ///
    /// If set, the browser will send this cookie to the set domain and all subdomains.
    /// If not set, the browser will only send this cookie to the originating domain.
    ///
    /// This may only be set to the sending domain and its subdomains.
    pub domain:     Option<String>,
    /// The scope of the cookie.
    pub path:       Option<String>,
    /// A cookie with this flag should only be sent over secured/encrypted connections.
    ///
    /// This will be respected by the browser.
    pub secure:     bool,
    /// A cookie with this flag is only accessible through HTTP and HTTPS.
    ///
    /// This helps to prevent Javascript and, specifically, XSS attacks.
    pub http_only:  bool,
    /// Any additional headers.
    ///
    /// This may be any sequence of valid characters.
    ///
    /// Extensions will be separated with `;`.
    /// If a value is specified in the `Map`, the extension will be
    /// written as `[key]=[value]`.
    pub extensions: Option<BTreeMap<String, Option<String>>>
}

impl HeaderCollection {
    #[doc(hidden)]
    pub fn to_cookie_av(self) -> String {
        // let mut options = String::new()
        //     .append(head("Expires", self.expires, |v| v.rfc822()).as_slice())
        //     .append(head("Max-Age", self.max_age, |v| v.to_String()).as_slice())
        //     .append(head("Domain", self.domain, |v| v).as_slice())
        //     .append(head("Path", self.path, |v| v).as_slice());
        // if self.secure { options.push_str("; Secure"); }
        // if self.http_only { options.push_str("; Http-Only"); }
        // match self.extensions {
        //     Some(map) => {
        //         for (header, value) in map.iter() {
        //             options.push_str(extension(header, value.clone()).as_slice());
        //         }
        //     },
        //     None      => ()
        // }
        // options
        "test".to_string()
    }
}

impl HeaderCollection {
    /// Convenience function for a set of empty cookie headers
    pub fn empty() -> HeaderCollection {
        HeaderCollection {
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            extensions: None
        }
    }

    /// Convenience function for a set of cookie headers
    /// that will expire the cookie in `seconds` seconds
    pub fn aged(seconds: u32) -> HeaderCollection {
        HeaderCollection {
            expires: None,
            max_age: Some(seconds),
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            extensions: None
        }
    }

    /// Convenience function for a set of cookie headers
    /// declaring the cookie `Secure` and `HttpOnly`
    pub fn secured() -> HeaderCollection {
        HeaderCollection {
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: true,
            http_only: true,
            extensions: None
        }
    }
}

//fn head<V>(header: &str, value: Option<V>, mutator: |V| -> String) -> String {
fn head<V>(header: &str, value: Option<V>, mutator: fn(&mut V) -> String) -> String {
    // match value {
    //     Some(val) => {
    //         // Delimit from previous cookie/options
    //         "; ".to_string()
    //         // Add the header
    //             .append(header).append("=")
    //         // Add the mutated value
    //             .append(mutator(val).as_slice())
    //     },
    //     None      => String::new()
    // }
    "test".to_string()
}

fn extension(header: &String, value: Option<String>) -> String {
    // match value {
    //     Some(val) => head(header.as_slice(), Some(val), |v| v),
    //     None      => "; ".to_String().append(header.as_slice())
    // }
    "test".to_string()
}
