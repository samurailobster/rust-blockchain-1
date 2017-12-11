#[macro_use]
extern crate prometheus;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate futures;

use futures::future::{FutureResult, ok};
use hyper::header::ContentType;
use hyper::mime::Mime;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Error, Method, StatusCode};

use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(
        opts!(
            "example_http_requests_total",
            "Total number of HTTP requests made.",
            labels!{"handler" => "all",}
        )
    ).unwrap();

    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(
        opts!(
            "example_http_response_size_bytes",
            "The HTTP response sizes in bytes.",
            labels!{"handler" => "all",}
        )
    ).unwrap();

    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    ).unwrap();
}

struct HelloWorld;

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = FutureResult<Response, Error>;

    fn call(&self, request: Request) -> Self::Future {
        let TEST: Counter = register_counter!(opts!(
            "example_without_static",
            "Nothing.",
            labels!{"handler" => "all",}
        )).unwrap();

        let encoder = TextEncoder::new();
        let mut response = Response::new();

        ok(match (request.method(), request.path()) {
            (&Method::Get, "/") => {
                let mut buffer = vec![];
                HTTP_COUNTER.inc();
                let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();

                let metric_familys = prometheus::gather();
                let lengthBuffer = buffer.clone().len();
                encoder.encode(&metric_familys, &mut buffer).unwrap();
                response.headers_mut().set(ContentType(
                    encoder
                        .format_type()
                        .parse::<Mime>()
                        .unwrap(),
                ));

                timer.observe_duration();
                HTTP_BODY_GAUGE.set(lengthBuffer as f64);
                response.with_body(buffer)
            }
            _ => response.with_status(StatusCode::NotFound),
        })
    }
}

fn main() {
    let addr = "127.0.0.1:9898";
    println!("listening addr {:?}", addr);
    let server = Http::new()
        .bind(&addr.parse().unwrap(), || Ok(HelloWorld))
        .unwrap();
    server.run().unwrap();
}
