use crate::infrastructure::shared::settings::Settings;

use ahash::AHashMap;
use std::any::Any;
use std::any::TypeId;

#[derive(Debug)]
pub struct State {
    pub settings: Settings,
    map: AHashMap<TypeId, Box<dyn Any>>,
}

unsafe impl Send for State {}
unsafe impl Sync for State {}

impl State {
    pub fn new(settings: Settings) -> State {
        State {
            settings,
            map: AHashMap::new(),
        }
    }

    pub async fn get<T: CreatedFromState + 'static>(&mut self) -> T {
        let item = self
            .map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>());

        if let Some(service) = item {
            return service.clone();
        }

        let service = T::create(&mut *self).await;

        self.map
            .insert(TypeId::of::<T>(), Box::new(service.clone()))
            .and_then(|boxed: Box<dyn Any>| -> Option<T> {
                boxed.downcast().ok().map(|boxed| *boxed)
            });

        service
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            settings: self.settings.clone(),
            map: AHashMap::new(),
        }
    }
}

#[async_trait::async_trait]
pub trait CreatedFromState: Clone + Send + Sync {
    async fn create(state: &mut State) -> Self;
}
