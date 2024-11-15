// #![warn(missing_docs)]

/// kernel is the main module that will be shared with all agents
/// Only export [`Kernel`] and [`KernelBuilder`] in this module
mod kernel;
pub use kernel::{Kernel, KernelBuilder};

mod builders;
// mod connectors;
// mod context_variables;
// mod converters;
// mod data;
mod filters;
// mod search; // will have text_search, vector_search
// mod storages; // vector_storage
// mod errors;
mod hooks;
// mod localizations;
mod plugins;
mod semantic_functions;
// mod orchestrations;
mod services; // audio, chat_completion, open_ai, text_completion, text_embedding,
              // mod template_engines; // handlebars, semantic_kernal
              // mod text;
