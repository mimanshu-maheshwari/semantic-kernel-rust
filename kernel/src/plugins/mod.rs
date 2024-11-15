use std::collections::HashMap;

use crate::semantic_functions::KernelFunction;

pub struct KernelPlugin{
    name: String, 
    description: String, 
    functions: HashMap<String, KernelFunction>
}
pub struct KernelPluginCollection{
    plugins: HashMap<String, KernelPlugin>,
}

impl KernelPluginCollection {

    pub fn new(plugins: Option<Vec<KernelPlugin>>) -> Self {
        let plugins = match plugins {
            Some(plugins) => HashMap::from_iter(plugins.into_iter().map(|plugin| (plugin.name.clone(), plugin))),
            None => HashMap::new(),
        };
        Self{ plugins }
    }
    
}
