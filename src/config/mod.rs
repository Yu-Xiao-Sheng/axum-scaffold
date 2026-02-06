// Configuration module
//
// This module contains project configuration structures and validation.

use serde::{Deserialize, Serialize};

/// Project configuration for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name (validated)
    pub project_name: String,

    /// Author name (optional)
    pub author_name: Option<String>,

    /// Project description
    pub description: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            project_name: "my-axum-app".to_string(),
            author_name: None,
            description: Some("An Axum web application".to_string()),
        }
    }
}
