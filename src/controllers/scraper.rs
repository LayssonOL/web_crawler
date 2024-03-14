use http::{Client, Request, Response};

pub fn scrape(url: &str, needs_awesome_header: bool) -> Response<()> {
    let mut request = Request::builder().uri(url);

    if needs_awesome_header {
        request = request.header("Awesome", "yes");
    }

    let response = send(request.body(()).unwrap());
    response
}

fn send(request: Request<()>) -> Response<()> {
    let response = http::Client::new().request(request).unwrap();
    Response::builder()
        .status(response.status())
        .body(())
        .unwrap()
}
