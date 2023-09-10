use crate::prelude::*;
use core::marker::PhantomData;

#[derive(Debug)]
pub struct Request {
    url: String,
    method: String,                 // more likely an enum, but flemme
    headers: Vec<(String, String)>, // (name, value)
    body: Option<String>,
}

// Can't use enums, enums are a single type, variants aren't types

// --- States

#[derive(Default, Clone)]
pub struct NoUrl;
#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct NoMethod;
#[derive(Default, Clone)]
pub struct Method(String);

#[derive(Default, Clone)]
pub struct NotSealed;

#[derive(Default, Clone)]
pub struct Sealed;

// --- States

#[derive(Default, Clone)]
pub struct RequestBuilder<U, M, S> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
    marker_seal: PhantomData<S>,
}

impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl<S> RequestBuilder<Url, Method, S> {
    pub fn build(self) -> Result<Request> {
        Ok(Request {
            url: self.url.0,
            method: self.method.0,
            headers: self.headers,
            body: self.body,
        })
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn seal(self) -> RequestBuilder<U, M, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        //#1 Generics -> Monomorphization
        // increase in binary size (& compilation time)
        // but more powerful code design
        // watch out when doing Embedded Systems

        //#2 moves are optimized at compile time
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }

    pub fn method(self, method: impl Into<String>) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
            marker_seal: PhantomData,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        let _ = self.body.insert(body.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}
