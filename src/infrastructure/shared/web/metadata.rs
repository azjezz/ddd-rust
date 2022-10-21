use actix_web::error::Error;
use actix_web::FromRequest;
use std::collections::HashMap;
use std::future::ready;
use std::future::Ready;

pub struct Metadata(pub HashMap<String, String>);

impl FromRequest for Metadata {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let mut metadata = HashMap::default();

        metadata.insert("URI".to_string(), req.uri().to_string());
        metadata.insert("Time".to_string(), chrono::Utc::now().to_rfc3339());

        let connection_info = req.connection_info();
        metadata.insert(
            "Connection-Scheme".to_string(),
            connection_info.scheme().to_string(),
        );
        metadata.insert(
            "Connection-Host".to_string(),
            connection_info.host().to_string(),
        );

        if let Some(address) = connection_info.realip_remote_addr() {
            metadata.insert("Connection-IP-Address".to_string(), address.to_string());
        }

        if let Some(address) = connection_info.peer_addr() {
            metadata.insert("Connection-Peer-Address".to_string(), address.to_string());
        }

        if let Some(header) = req.headers().get("User-Agent") {
            if let Ok(user_agent) = header.to_str() {
                metadata.insert("User-Agent".to_string(), user_agent.to_string());
            }
        }

        ready(Ok(Metadata(metadata)))
    }
}
