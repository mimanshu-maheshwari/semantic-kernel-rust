use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// A collection of AI services.
pub struct AIServiceCollection {
    map: HashMap<TypeId, Box<dyn AIService>>,
}

impl AIServiceCollection {
    /// Creates a new, empty `AIServiceCollection`.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// Adds a service to the collection.
    pub fn insert<S: AIService + 'static + Clone>(&mut self, service: S) {
        self.map.insert(TypeId::of::<S>(), Box::new(service));
    }

    /// Retrieves a service by type, if it exists.
    pub fn get<S: AIService + 'static + Clone>(&self) -> Option<&S> {
        self.map.get(&TypeId::of::<S>()).and_then(|service| {
            service.as_ref().as_any().downcast_ref::<S>()
        })
    }

    /// Retrieves the model ID for a service of a specific type, if it exists.
    pub fn get_model_id<S: AIService + 'static + Clone>(&self) -> Option<String> {
        self.get::<S>().and_then(|service| service.get_model_id())
    }

    /// Retrieves the service ID for a service of a specific type, if it exists.
    pub fn get_service_id<S: AIService + 'static + Clone>(&self) -> Option<String> {
        self.get::<S>().and_then(|service| service.get_service_id())
    }
}

/// Trait representing an AI service.
pub trait AIService {
    fn get_model_id(&self) -> Option<String>; 
    fn get_service_id(&self) -> Option<String>;
    fn as_any(&self) -> &dyn Any;
}


pub trait AIServiceSelector{
    fn try_select_service(&self);
}

pub struct BaseAIServiceSelector{
    services: Arc<AIServiceCollection>,
}

impl AIServiceSelector for BaseAIServiceSelector {
    fn try_select_service(&self) {
        unimplemented!();
    }
}
impl BaseAIServiceSelector {
    pub fn new(services: Arc<AIServiceCollection>) -> Self {
        Self{ services: Arc::clone(&services) }
    }
    
}
pub struct OrderedAIServiceSelector {
    services: Arc<AIServiceCollection>,
}

impl AIServiceSelector for OrderedAIServiceSelector {
    fn try_select_service(&self) {
        unimplemented!();
    }
}

impl OrderedAIServiceSelector {
    pub fn new(services: Arc<AIServiceCollection>) -> Self {
        Self{ services: Arc::clone(&services) }
    }
}
