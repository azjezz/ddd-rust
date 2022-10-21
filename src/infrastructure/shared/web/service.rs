use crate::infrastructure::shared::state::{CreatedFromState, State};

use actix_web::web::Data;
use actix_web::Error;
use actix_web::FromRequest;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;

pub struct Service<T: CreatedFromState + 'static>(pub T);

impl<T: CreatedFromState + 'static> Service<T> {
    pub fn new(instance: T) -> Self {
        Service(instance)
    }
}

impl<T: CreatedFromState + 'static> FromRequest for Service<T> {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let state = req.app_data::<Data<Mutex<State>>>().unwrap();
            let service: T = state.lock().await.get::<T>().await;

            Ok(Service::new(service))
        })
    }
}
