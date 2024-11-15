use std::sync::Arc;

use crate::{
    // builders::SemanticKernelBuilder,
    hooks::KernelHooks,
    plugins::{KernelPlugin, KernelPluginCollection},
    services::{AIService, AIServiceCollection, AIServiceSelector, OrderedAIServiceSelector}
};

/// Provides state for use throughout a Semantic Kernel workload.
/// An instance of `Kernel` is passed through to every function invocation and service call
/// throughout the system, providing to each the ability to access shared state and services.
pub struct Kernel {
    service_selector: Box<dyn AIServiceSelector>,
    services: Arc<AIServiceCollection>,
    service_selector_provider: Option<fn(&AIServiceCollection) -> Box<dyn AIServiceSelector>>,
    plugins: Arc<KernelPluginCollection>,
    global_kernel_hooks: Arc<KernelHooks>,
}

impl Kernel {

    /// Creates a new instance of `Kernel`.
    ///
    /// # Arguments
    /// * `service` - Represents the collection of AI services available to the kernel.
    /// * `service_selector_provider` - An optional provider for selecting services from the collection.
    ///   If `None`, an ordered service selector will be used as the default.
    /// * `plugins` - An optional collection of plugins to be registered with the kernel.
    ///   If `None`, the kernel will initialize with an empty plugin collection.
    /// * `global_kernel_hooks` - Optional global hooks that apply to the kernel's operations.
    ///   If `None`, the kernel will operate without global hooks.
    ///
    /// # Returns
    /// A new instance of the `Kernel` struct.
    pub fn new(
        services: AIServiceCollection,
        service_selector_provider: Option<fn(&AIServiceCollection) -> Box<dyn AIServiceSelector>>,
        plugins: Option<Vec<KernelPlugin>>,
        global_kernel_hooks: Option<KernelHooks>,
    ) -> Self {
        let services = Arc::new(services);
        let service_selector = match service_selector_provider {
            Some(service_selector_provider) => {
                service_selector_provider(&services)
            },
            None => {
                Box::new(OrderedAIServiceSelector::new(Arc::clone(&services)))
            },
        };
        let plugins = Arc::new(KernelPluginCollection::new(plugins)); 
        let global_kernel_hooks = Arc::new(KernelHooks::from_optional(global_kernel_hooks));
        Self {services, service_selector, service_selector_provider, plugins, global_kernel_hooks}
    }
}
/// A fluent builder for creating a new instance of [`Kernel`].
pub struct KernelBuilder{
    services: AIServiceCollection,
    service_selector_provider: Option<fn(&AIServiceCollection) -> Box<dyn AIServiceSelector>>,
    plugins: Vec<KernelPlugin>,
}

impl KernelBuilder {

    /// Construct a Builder for creating a new instance of [`Kernel`]. 
    pub fn new() -> Self {
        Self{services: AIServiceCollection::new(), service_selector_provider:None, plugins: Vec::new() }
    }

    /// Adds a service to the kernel.
    pub fn with_ai_service<S: AIService + 'static + Clone>(&mut self, service: S) -> &mut Self {
        self.services.insert(service);
        self
    }

    /// Adds a plugin to the kernel.
    pub fn with_plugin(&mut self, plugin: KernelPlugin) -> &mut Self {
        self.plugins.push(plugin);
        self
    }

    /// Sets the service selector provider for the kernel.
    pub fn withServiceSelector(&mut self, service_selector: fn(&AIServiceCollection) -> Box<dyn AIServiceSelector>) -> &mut Self {
        self.service_selector_provider = Some(service_selector);
        self
    }

    /// Builds a new instance of {@code Kernel} with the services and plugins provided.
    pub fn build(self) -> Kernel {
        Kernel::new(self.services, self.service_selector_provider, Some(self.plugins), None)
    }
}

// impl SemanticKernelBuilder<Kernel> for KernelBuilder {
//     fn build(&self) -> Kernel {
//         todo!();
//     }
// }
