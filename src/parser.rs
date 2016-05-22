//! Parsing functionality - get cookie data

use std::collections::BTreeMap;
use url::Url;
use iron::{Request, Response, BeforeMiddleware, IronResult, Handler};
use iron::status::{Status, Continue};
use super::Cookie;
use crypto::util::fixed_time_eq;
use regex::Regex;
use serde_json;

/// The cookie parsing `Middleware`.
///
/// It will parse the body of a cookie into the extensions, under type `Cookie`.
///
/// This middleware should be linked (added to the `Chain`)
/// before any other middleware using cookies, or the parsed cookie
/// will not be available to that middleware.
#[derive(Clone)]
pub struct CookieParser {
    secret: Option<String>
}

impl CookieParser {
    /// Create a new instance of the cookie parsing `Middleware`.
    ///
    /// This instance will parse both RFC 6265-styled cookies:
    /// `key=value; key=value;`
    /// and json-styled cookies, as set with `res.set_json_cookie(...)`.
    pub fn new() -> CookieParser { CookieParser{ secret: None} }

    /// Create a cookie parser with secret, for signed cookies.
    ///
    /// This instance will parse any cookies that have been signed by
    /// you, or that are unsigned. It will not parse those cookies signed by others.
    ///
    /// Otherwise, it will behave exactly like that produced by `new`.
    pub fn signed(secret: String) -> CookieParser { CookieParser{ secret: Some(secret) } }
}

impl BeforeMiddleware for CookieParser {
    /// Parse the cookie received in the HTTP header.
    ///
    /// This will parse the body of a cookie into the extensions, under type `Cookie`.
    fn before(&self, req: &mut Request) -> IronResult<()> {
    //fn enter(&mut self, req: &mut Request, _res: &mut Response) -> Status {
        // Initialize a cookie. This will store parsed cookies and generate signatures.
        let mut new_cookie = Cookie::new(self.secret.clone());

        match req.headers.get_mut::<Cookie>() {
        //match req.headers.get_mut::<Cookie>() {
            Some(cookies) => {
                // // Initialize an empty json object.
                // //let mut new_json = serde_json::Object(TreeMap::new());
                // let mut new_json: BTreeMap<String, serde_json::Value> = BTreeMap::new();
                // new_cookie.map =
                //     cookies
                //         .as_slice()
                //         .split(';')
                //         // Decode from uri component encoding
                //         .map(|substr| {
                //             let vec: Vec<&str> = substr.splitn('=', 1).collect();
                //             let key = from_rfc_compliant(vec[0]);
                //             let val = from_rfc_compliant(vec[1]);
                //             (key, val) })
                //         // Check for signed cookies, and filter those not signed by us
                //         .filter_map(|cookie| strip_signature(cookie, &new_cookie))
                //         // Move json cookies into a separate container
                //         //.filter(|cookie| parse_json(cookie, &mut new_json))
                //         .filter(|cookie| new_json = serde_json::from_str(&cookie).unwrap())
                //         .collect();

                // // This cannot be inserted via iterators because strip_signature
                // // is already borrowing new_cookie.
                // new_cookie.json = new_json;

                // let  mut new_json: BTreeMap<String, serde_json::Value> = BTreeMap::new();
                // new_cookie.map = cookies.map;
                // new_cookie.json = new_json;
                
            },
            None => ()
        }
        
        req.extensions.insert::<Cookie>(new_cookie);
        Ok(())
    }
}

fn from_rfc_compliant(string: &str) -> String {
    // Url::percent_decode(
    //     string
    //         .chars()
    //         .skip_while(is_whitespace)
    //         .collect::<String>()
    //         .as_bytes()
    // )
    "".to_string()
}

fn is_whitespace(c: &char) -> bool {
    match *c {
        ' '|'\r'|'\t'|'\n' => true,
        _                  => false
    }
}

fn strip_signature((key, val): (String, String), signer: &Cookie) -> Option<(String, String)> {
    // if val.len() > 2 && val.as_slice().slice(0, 2) == "s:" {
    //     if !signer.signed { return None }
    //     // Extract the signature (in hex), appended onto the cookie after `.`
    //     let re = Regex::new(r"\.[^\.]*$").unwrap();
    //     return re.is_match(val.as_slice())
    //         // If it was signed by us, clear the signature
    //         .and_then(|(beg, end)| {
    //             signer.sign(&val.as_slice().slice(2, beg).to_string())
    //                 // We need to maintain access to (beg, end), so we chain the signature
    //                 .and_then(|signature| {
    //                     // If the signature is valid, strip it
    //                      if fixed_time_eq(val.as_slice().slice(beg + 1, end).as_bytes(), signature.as_bytes()) {
    //                         // key must be cloned to move out of the closure capture
    //                         Some((key.clone(), val.as_slice().slice(2, beg).to_string()))
    //                     // Else, remove the cookie
    //                     } else {
    //                         None
    //                     }
    //                 })
    //         })
    // }
    // match signer.signed {
    //     true => None,
    //     false => Some((key, val))
    // }
    None
}
