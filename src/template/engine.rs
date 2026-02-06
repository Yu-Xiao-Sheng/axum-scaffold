// Template engine
//
// This module sets up Handlebars for template rendering.

use crate::error::{CliError, Result};
use crate::template::context::TemplateContext;
use handlebars::Handlebars;

/// Template rendering engine
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    /// Create a new template engine with strict mode enabled
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        // Enable strict mode to catch undefined variables at render time
        handlebars.set_strict_mode(true);

        // Register custom helpers
        register_custom_helpers(&mut handlebars);

        Self { handlebars }
    }

    /// Render a template string with the given context
    ///
    /// # Arguments
    /// * `template_content` - The raw template string (e.g., from include_str!)
    /// * `context` - The template context containing variables
    ///
    /// # Returns
    /// * `Ok(String)` containing the rendered output
    /// * `Err(CliError)` if rendering fails
    pub fn render_template(
        &self,
        template_name: &str,
        template_content: &str,
        context: &TemplateContext,
    ) -> Result<String> {
        self.handlebars
            .render_template(template_content, context)
            .map_err(|e| CliError::Template(format!("{}: {}", template_name, e)))
    }

    /// Register a template string from memory
    ///
    /// # Arguments
    /// * `name` - Template name for later reference
    /// * `content` - Template content
    pub fn register_template_string(&mut self, name: &str, content: &str) -> Result<()> {
        self.handlebars
            .register_template_string(name, content)
            .map_err(|e| CliError::Template(format!("Failed to register template '{}': {}", name, e)))
    }

    /// Render a registered template by name
    pub fn render(&self, name: &str, context: &TemplateContext) -> Result<String> {
        self.handlebars
            .render(name, context)
            .map_err(|e| CliError::Template(format!("Failed to render template '{}': {}", name, e)))
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Register custom Handlebars helpers
fn register_custom_helpers(handlebars: &mut Handlebars) {
    use handlebars::{Output, RenderErrorReason};

    // Helper: to_snake_case
    // Converts a string to snake_case
    handlebars.register_helper(
        "to_snake_case",
        Box::new(
            |h: &handlebars::Helper<'_>,
             _r: &handlebars::Handlebars<'_>,
             _: &handlebars::Context,
             _rc: &mut handlebars::RenderContext<'_, '_>,
             out: &mut dyn Output|
             -> handlebars::HelperResult {
                let param = h
                    .param(0)
                    .ok_or_else(|| RenderErrorReason::Other("Missing parameter for to_snake_case".into()))?;
                let value = param
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderErrorReason::Other("Parameter must be a string".into()))?;
                let result = to_snake_case(value);
                out.write(&result)?;
                Ok(())
            },
        ),
    );

    // Helper: to_pascal_case
    // Converts a string to PascalCase
    handlebars.register_helper(
        "to_pascal_case",
        Box::new(
            |h: &handlebars::Helper<'_>,
             _r: &handlebars::Handlebars<'_>,
             _: &handlebars::Context,
             _rc: &mut handlebars::RenderContext<'_, '_>,
             out: &mut dyn Output|
             -> handlebars::HelperResult {
                let param = h
                    .param(0)
                    .ok_or_else(|| RenderErrorReason::Other("Missing parameter for to_pascal_case".into()))?;
                let value = param
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderErrorReason::Other("Parameter must be a string".into()))?;
                let result = to_pascal_case(value);
                out.write(&result)?;
                Ok(())
            },
        ),
    );

    // Helper: to_upper_camel_case (alias for to_pascal_case)
    handlebars.register_helper(
        "to_upper_camel_case",
        Box::new(
            |h: &handlebars::Helper<'_>,
             _r: &handlebars::Handlebars<'_>,
             _: &handlebars::Context,
             _rc: &mut handlebars::RenderContext<'_, '_>,
             out: &mut dyn Output|
             -> handlebars::HelperResult {
                // Reuse to_pascal_case implementation
                let param = h
                    .param(0)
                    .ok_or_else(|| RenderErrorReason::Other("Missing parameter for to_upper_camel_case".into()))?;
                let value = param
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderErrorReason::Other("Parameter must be a string".into()))?;
                let result = to_pascal_case(value);
                out.write(&result)?;
                Ok(())
            },
        ),
    );
}

/// Convert kebab-case to snake_case
fn to_snake_case(name: &str) -> String {
    name.replace('-', "_")
}

/// Convert kebab-case to PascalCase
fn to_pascal_case(name: &str) -> String {
    name.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + chars.as_str()
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("my-app"), "my_app");
        assert_eq!(to_snake_case("my_axum_app"), "my_axum_app");
        assert_eq!(to_snake_case("myapp"), "myapp");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("my-app"), "MyApp");
        assert_eq!(to_pascal_case("my-axum-app"), "MyAxumApp");
        assert_eq!(to_pascal_case("myapp"), "Myapp");
    }

    #[test]
    fn test_template_engine_creation() {
        let engine = TemplateEngine::new();
        // Basic test - just ensure it doesn't panic
        assert!(engine.handlebars.strict_mode());
    }
}
