use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct ReplaceServerHeader;

impl ReplaceServerHeader {
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn get_server_name() -> &'static str {
        "jens1o"
    }

    #[cfg(debug_assertions)]
    #[inline(always)]
    fn get_server_name() -> &'static str {
        "jens1o [DEBUG]"
    }
}

impl Fairing for ReplaceServerHeader {
    fn info(&self) -> Info {
        Info {
            name: "Replace Server header",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new(
            "Server",
            ReplaceServerHeader::get_server_name(),
        ));
    }
}
