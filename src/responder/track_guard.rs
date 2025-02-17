// Raider
//
// Affiliates dashboard
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use base64;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use APP_CONF;

pub struct TrackGuard;

pub struct Authorization {
    pub username: String,
    pub password: String,
}

impl Authorization {
    pub fn parse_from(scheme: &str, key: &str) -> Result<Authorization, ()> {
        let start_from = scheme.len() + 1;

        if key.starts_with(scheme) && key.len() > start_from {
            base64::decode(&key[start_from..])
                .or(Err(()))
                .and_then(|decoded| String::from_utf8(decoded).or(Err(())))
                .and_then(|text| {
                    let parts: Vec<&str> = text.split(":").collect();

                    if parts.len() == 2 {
                        Ok(Authorization {
                            username: parts[0].to_owned(),
                            password: parts[1].to_owned(),
                        })
                    } else {
                        Err(())
                    }
                })
        } else {
            Err(())
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for TrackGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<TrackGuard, ()> {
        if let Some(authorization_value) = request.headers().get_one("authorization") {
            match Authorization::parse_from("Basic", authorization_value) {
                Ok(authorization) => {
                    if authorization.password == APP_CONF.server.track_token {
                        Outcome::Success(TrackGuard)
                    } else {
                        Outcome::Failure((Status::Unauthorized, ()))
                    }
                }
                Err(_) => Outcome::Failure((Status::BadRequest, ())),
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
