use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// 模板元数据结构（用于解析 template.toml）
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateMeta {
    pub template: TemplateInfo,
    pub variables: Option<HashMap<String, VariableDef>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VariableDef {
    pub prompt: Option<String>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSummary {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverwriteMode {
    Fail,
    Overwrite,
    Skip,
}

#[derive(Debug, Clone)]
pub struct InitOptions {
    pub template_name: String,
    pub output_base: PathBuf,
    pub variables: HashMap<String, String>,
    pub overwrite: OverwriteMode,
    pub dry_run: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InitReport {
    pub created_dirs: Vec<PathBuf>,
    pub created_files: Vec<PathBuf>,
    pub skipped_files: Vec<PathBuf>,
}
