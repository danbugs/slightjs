// <http-server>
/// The HTTP status code.
pub type HttpStatus = u16;
/// The HTTP body.
pub type BodyString = String;
/// The HTTP method.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}
// </http-server>

// <http-client>
/// The HTTP URI of the current request.
pub type Uri = String;
/// The HTTP parameter queries, represented as a list of (name, value) pairs.
pub type Params = Vec<(String, String)>;
/// The HTTP body.
pub type BodyVec = Vec<u8>;
/// An HTTP request.
#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: Headers,
    pub params: Params,
    pub body: Option<BodyVec>,
}

// </http-client>

// <shared>
/// An HTTP response.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub status: HttpStatus,
    pub headers: Option<Headers>,
    pub body: Option<BodyString>,
}
/// The HTTP headers represented as a list of (name, value) pairs.
pub type Headers = Vec<(String, String)>;
// </shared>