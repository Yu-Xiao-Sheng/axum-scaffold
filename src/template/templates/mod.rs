// Embedded templates
//
// This module contains all embedded project templates.
//
// Templates are embedded at compile time using include_str! macro
// This allows the CLI tool to work offline after installation

use std::collections::HashMap;

/// Template file descriptor
#[derive(Debug, Clone)]
pub struct TemplateFile {
    /// Relative path in generated project
    pub path: &'static str,
    /// Template content (embedded via include_str!)
    pub content: &'static str,
    /// Is the file executable? (for scripts)
    pub executable: bool,
}

/// Get all templates for single-package mode
///
/// This function returns a map of template paths to their contents.
/// In Phase 2 (foundational), this returns a minimal set.
/// User Story 1 will populate this with actual templates.
pub fn get_single_mode_templates() -> HashMap<&'static str, TemplateFile> {
    let templates = HashMap::new();

    // Templates will be added in User Story 1
    // For now, this is a placeholder

    templates
}

/// Get all template files
///
/// Returns a vector of all template file descriptors
pub fn get_all_templates() -> Vec<TemplateFile> {
    vec![
        // Placeholder - templates will be added in User Story 1
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_single_mode_templates() {
        let templates = get_single_mode_templates();
        // For now, just ensure it doesn't panic
        // In User Story 1, this will contain actual templates
        let _ = templates;
    }

    #[test]
    fn test_get_all_templates() {
        let templates = get_all_templates();
        // For now, just ensure it returns a vector
        let _ = templates;
    }
}
