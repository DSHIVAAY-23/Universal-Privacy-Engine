//! # MCP (Model Context Protocol) Module
//!
//! Provides MCP server for Cursor/Claude integration.
//! Exposes tools for automated ZK proof generation.

pub mod server;
pub mod tools;

pub use server::VeriVaultMcpServer;
pub use tools::{ToolDefinition, ToolResult};
