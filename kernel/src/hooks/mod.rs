use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KernelHook;
#[derive(Debug, Clone)]
pub struct KernelHooks{
    hooks: HashMap<String, KernelHook>
}

impl KernelHooks {

    pub fn new() -> Self {
        Self{ hooks: HashMap::new() }
    }

    pub fn from_map(hooks: &HashMap<String, KernelHook>) -> Self {
        Self{ hooks: hooks.clone() }
    }
    
    pub fn from_optional(hooks: Option<KernelHooks>) -> Self {
        let hooks = match hooks {
            Some(hooks) => hooks.hooks.clone(), 
            None => HashMap::new(),
        };
        Self{ hooks }
    }

}


