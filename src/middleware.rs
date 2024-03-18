use chrono::{DateTime, Utc};
use rocket::fairing::Fairing;
use rocket::request::{FromRequest, Outcome};
use rocket::{Data, Request, Response};
use tracing::info;
use uuid::Uuid;

pub struct RequestID(pub String);
/// Value stored in request-local state.
#[derive(Copy, Clone)]
struct TimerStart(Option<DateTime<Utc>>);

// Implementation of the request guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestID {
    type Error = ();

    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestID(Uuid::new_v4().to_string()))
    }
}

pub struct RequestLogger;

// Middleware to attach the request metadata to each request
#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Request Metadata Middleware",
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let now = Utc::now();
        info!(name: "Request started",
            start_time = now.timestamp(),
            path = &req.uri().to_string(),
            method = &req.method().to_string(),
        );
        req.local_cache(|| TimerStart(Some(now)));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let end_time = Utc::now();
        let start_time = req.local_cache(|| TimerStart(None));

        // Calculate the duration
        let duration = match start_time.0 {
            Some(start) => end_time - start,
            None => {
                println!("Start time is not initialized.");
                Default::default() // Or handle the case when start_time is None
            }
        };
        info!(name: "Request finished",
            status = res.status().to_string(),
            end_time = end_time.timestamp(),
            duration = duration.num_milliseconds()
        );
    }
}
