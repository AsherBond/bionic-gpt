//! Integrations crate
//!
//! This crate provides integration with external services and tools.

pub mod external_integration;
pub mod tool;
pub mod tool_executor;
pub mod tool_registry;
pub mod tools;

#[cfg(test)]
mod test_async;

// Re-export key types for convenience
pub use external_integration::{
    create_tool_definitions_from_spec, create_tools_from_integration,
    create_tools_from_integrations, extract_base_url, ExternalIntegrationTool, IntegrationTools,
};
pub use tool::ToolInterface;
pub use tool_executor::{execute_tool_call_with_tools, execute_tool_calls};
pub use tool_registry::{
    get_chat_tools_user_selected, get_integrations, get_tools_for_attachments,
    get_user_selectable_tools_for_chat_ui, IntegrationTool, ToolScope,
};
