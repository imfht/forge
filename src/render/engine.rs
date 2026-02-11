use std::collections::HashMap;
use std::path::Path;
use tera::{Function, Result as TeraResult, Tera, Value};

use crate::error::ForgeResult;

pub fn create_tera_engine(site_dir: &Path, theme: &str) -> ForgeResult<Tera> {
    let mut tera = Tera::default();

    // Load theme templates first
    let theme_templates_dir = site_dir.join("themes").join(theme).join("templates");
    if theme_templates_dir.exists() {
        let pattern = format!("{}/**/*.html", theme_templates_dir.display());
        let theme_tera = Tera::new(&pattern)?;
        tera.extend(&theme_tera)?;
    }

    // Load site-level template overrides
    let site_templates_dir = site_dir.join("templates");
    if site_templates_dir.exists() {
        let pattern = format!("{}/**/*.html", site_templates_dir.display());
        let site_tera = Tera::new(&pattern)?;
        tera.extend(&site_tera)?;
    }

    // Disable auto-escaping since we control all template variables
    tera.autoescape_on(vec![]);

    // Register custom filters
    tera.register_filter("date_format", date_format_filter);
    tera.register_filter("truncate_words", truncate_words_filter);

    Ok(tera)
}

pub fn register_functions(
    tera: &mut Tera,
    base_url: String,
    translations: HashMap<String, HashMap<String, String>>,
    default_lang: String,
) {
    tera.register_function(
        "get_url",
        GetUrlFunction {
            base_url: base_url.clone(),
        },
    );
    tera.register_function("get_taxonomy_url", GetTaxonomyUrlFunction { base_url });
    tera.register_function(
        "trans",
        TransFunction {
            translations,
            default_lang,
        },
    );
}

/// Filter: format a date string
fn date_format_filter(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let date_str = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("date_format: expected string value"))?;

    let format = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%B %d, %Y");

    let date = chrono::DateTime::parse_from_rfc3339(date_str)
        .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.fZ"))
        .or_else(|_| chrono::DateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%SZ"))
        .map_err(|e| tera::Error::msg(format!("date_format: {e}")))?;

    Ok(Value::String(date.format(format).to_string()))
}

/// Filter: truncate text to N words
fn truncate_words_filter(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let text = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("truncate_words: expected string"))?;

    let count = args.get("count").and_then(|v| v.as_u64()).unwrap_or(50) as usize;

    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() <= count {
        Ok(Value::String(text.to_string()))
    } else {
        let truncated = words[..count].join(" ");
        Ok(Value::String(format!("{truncated}...")))
    }
}

struct GetUrlFunction {
    base_url: String,
}

impl Function for GetUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let path = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("get_url: missing 'path' argument"))?;

        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        Ok(Value::String(format!("{base}/{path}")))
    }

    fn is_safe(&self) -> bool {
        true
    }
}

struct GetTaxonomyUrlFunction {
    base_url: String,
}

impl Function for GetTaxonomyUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let taxonomy = args
            .get("taxonomy")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("get_taxonomy_url: missing 'taxonomy' argument"))?;

        let term = args.get("term").and_then(|v| v.as_str());

        let base = self.base_url.trim_end_matches('/');
        let tax_slug = slug::slugify(taxonomy);

        let url = if let Some(term) = term {
            let term_slug = slug::slugify(term);
            format!("{base}/{tax_slug}/{term_slug}/")
        } else {
            format!("{base}/{tax_slug}/")
        };

        Ok(Value::String(url))
    }

    fn is_safe(&self) -> bool {
        true
    }
}

struct TransFunction {
    translations: HashMap<String, HashMap<String, String>>,
    default_lang: String,
}

impl Function for TransFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let key = args
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| tera::Error::msg("trans: missing 'key' argument"))?;

        let lang = args
            .get("lang")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.default_lang);

        let translated = self
            .translations
            .get(lang)
            .and_then(|t| t.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string());

        // Simple variable substitution: %{var}
        let mut result = translated;
        for (arg_key, arg_val) in args {
            if arg_key != "key" && arg_key != "lang" {
                if let Some(val_str) = arg_val.as_str() {
                    result = result.replace(&format!("%{{{arg_key}}}"), val_str);
                } else {
                    result = result.replace(&format!("%{{{arg_key}}}"), &arg_val.to_string());
                }
            }
        }

        Ok(Value::String(result))
    }

    fn is_safe(&self) -> bool {
        true
    }
}
