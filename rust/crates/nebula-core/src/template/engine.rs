use anyhow::{Context, Result, bail};
use include_dir::{Dir, DirEntry, include_dir};
use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use tera::{Context as TeraContext, Tera};

use nebula_protos::{InitOptions, InitReport, OverwriteMode, TemplateMeta, TemplateSummary};

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_WORKSPACE_DIR/assets/templates");

/// 解析嵌入的所有模板
pub fn list_template() -> Result<Vec<TemplateSummary>> {
    let mut templates = Vec::new();

    for entry in TEMPLATE_DIR.entries() {
        if let Some(dir) = entry.as_dir() {
            let name = dir
                .path()
                .file_name()
                .and_then(|name| name.to_str())
                .context("模板目录名称不是有效的 UTF-8")?
                .to_string();

            match load_template_meta(&name) {
                Ok(meta) => templates.push(TemplateSummary {
                    name: meta.template.name,
                    description: Some(meta.template.description),
                    version: Some(meta.template.version),
                }),
                Err(_) => templates.push(TemplateSummary {
                    name,
                    description: None,
                    version: None,
                }),
            }
        }
    }

    templates.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(templates)
}

/// 加载模板元数据
pub fn load_template_meta(template_name: &str) -> Result<TemplateMeta> {
    let file_path = format!("{}/template.toml", template_name);
    let file = TEMPLATE_DIR
        .get_file(&file_path)
        .with_context(|| format!("模板 '{}' 中缺少 template.toml", template_name))?;
    let content = file
        .contents_utf8()
        .context("template.toml 不是有效的 UTF-8")?;
    toml::from_str(content).context("无法解析 template.toml")
}

pub fn build_context(
    meta: &TemplateMeta,
    provided: &HashMap<String, String>,
) -> Result<TeraContext> {
    let mut context = TeraContext::new();
    let mut variable_names = template_variable_names(meta, provided);
    variable_names.sort();

    for name in variable_names {
        let value = provided
            .get(&name)
            .or_else(|| provided.get(&to_safe_key(&name)))
            .cloned()
            .or_else(|| {
                meta.variables
                    .as_ref()
                    .and_then(|variables| variables.get(&name))
                    .and_then(|def| def.default.clone())
            })
            .or_else(|| {
                meta.variables
                    .as_ref()
                    .and_then(|variables| variables.get(&to_original_key(&name)))
                    .and_then(|def| def.default.clone())
            })
            .with_context(|| format!("缺少变量 '{}'，请通过 --var 提供", name))?;

        validate_variable_value(&name, &value)?;
        insert_aliases(&mut context, &name, &value);
    }

    Ok(context)
}

pub fn initialize_project(options: InitOptions) -> Result<InitReport> {
    let template_dir = TEMPLATE_DIR
        .get_dir(&options.template_name)
        .with_context(|| format!("模板 '{}' 不存在", options.template_name))?;
    let meta = load_template_meta(&options.template_name)?;
    let variable_names = template_variable_names(&meta, &options.variables);
    let context = build_context(&meta, &options.variables)?;

    let mut report = InitReport::default();
    process_template_dir(
        template_dir,
        template_dir,
        &options.output_base,
        &context,
        &variable_names,
        &options,
        &mut report,
    )?;

    Ok(report)
}

fn process_template_dir(
    template_root: &Dir,
    current_dir: &Dir,
    output_base: &Path,
    context: &TeraContext,
    variable_names: &[String],
    options: &InitOptions,
    report: &mut InitReport,
) -> Result<()> {
    for entry in current_dir.entries() {
        let relative_path = entry
            .path()
            .strip_prefix(template_root.path())
            .with_context(|| format!("无法计算模板相对路径 '{}'", entry.path().display()))?;

        if relative_path == Path::new("template.toml") || relative_path.as_os_str().is_empty() {
            continue;
        }

        let rendered_relative = render_path(relative_path, context, variable_names)?;
        let target_path = safe_join(output_base, &rendered_relative)?;

        match entry {
            DirEntry::Dir(dir) => {
                if !options.dry_run {
                    fs::create_dir_all(&target_path)
                        .with_context(|| format!("无法创建目录 '{}'", target_path.display()))?;
                }
                report.created_dirs.push(target_path);
                process_template_dir(
                    template_root,
                    dir,
                    output_base,
                    context,
                    variable_names,
                    options,
                    report,
                )?;
            },
            DirEntry::File(file) => {
                if target_path.exists() {
                    match options.overwrite {
                        OverwriteMode::Fail => bail!(
                            "目标文件已存在：{}。如需覆盖，请使用 --force",
                            target_path.display()
                        ),
                        OverwriteMode::Overwrite => {},
                        OverwriteMode::Skip => {
                            report.skipped_files.push(target_path);
                            continue;
                        },
                    }
                }

                if !options.dry_run {
                    if let Some(parent) = target_path.parent() {
                        fs::create_dir_all(parent)
                            .with_context(|| format!("无法创建目录 '{}'", parent.display()))?;
                    }

                    if let Some(raw_content) = file.contents_utf8() {
                        let rendered_content = render_string(raw_content, context, variable_names)?;
                        write_file(&target_path, rendered_content.as_bytes(), options.overwrite)?;
                    }
                }

                report.created_files.push(target_path);
            },
        }
    }

    Ok(())
}

fn write_file(path: &Path, content: &[u8], overwrite: OverwriteMode) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true);

    match overwrite {
        OverwriteMode::Fail | OverwriteMode::Skip => {
            options.create_new(true);
        },
        OverwriteMode::Overwrite => {
            options.create(true).truncate(true);
        },
    }

    let mut file = options
        .open(path)
        .with_context(|| format!("无法写入文件：{}", path.display()))?;
    file.write_all(content)
        .with_context(|| format!("无法写入文件：{}", path.display()))?;
    Ok(())
}

fn render_path(path: &Path, context: &TeraContext, variable_names: &[String]) -> Result<PathBuf> {
    let raw = path
        .to_str()
        .with_context(|| format!("模板路径不是有效的 UTF-8 : {}", path.display()))?;
    let rendered = render_string(raw, context, variable_names)?;
    let rendered_path = PathBuf::from(rendered);
    validate_relative_path(&rendered_path)?;
    Ok(rendered_path)
}

/// 使用 Tera 引擎渲染单个字符串（路径或文本内容）
fn render_string(
    template: &str,
    context: &TeraContext,
    variable_names: &[String],
) -> Result<String> {
    let normalized = normalize_template_syntax(template, variable_names);
    Tera::one_off(&normalized, context, false).context("模板渲染失败")
}

fn normalize_template_syntax(template: &str, variable_names: &[String]) -> String {
    let mut normalized = template.to_string();

    for variable_name in variable_names {
        if !variable_name.contains('-') {
            continue;
        }

        let safe_key = to_safe_key(variable_name);
        let patterns = [
            format!("{{{{{}}}}}", variable_name),
            format!("{{{{ {} }}}}", variable_name),
            format!("{{{{{} }}}}", variable_name),
            format!("{{{{ {}}}}}", variable_name),
        ];

        for pattern in patterns {
            normalized = normalized.replace(&pattern, &format!("{{{{ {} }}}}", safe_key));
        }
    }

    normalized
}

fn safe_join(base: &Path, relative: &Path) -> Result<PathBuf> {
    validate_relative_path(relative)?;
    Ok(base.join(relative))
}

fn validate_relative_path(path: &Path) -> Result<()> {
    for component in path.components() {
        match component {
            Component::Normal(_) | Component::CurDir => {},
            Component::ParentDir => bail!("渲染后的路径不能包含 '..': {}", path.display()),
            Component::RootDir | Component::Prefix(_) => {
                bail!("渲染后的路径不能是绝对路径：{}", path.display())
            },
        }
    }

    Ok(())
}

fn validate_variable_value(name: &str, value: &str) -> Result<()> {
    let name = to_original_key(name);
    if !is_path_like_variable(&name) {
        return Ok(());
    }

    if value.trim().is_empty() {
        bail!("变量 '{}' 不能为空", name);
    }

    if value == "." || value == ".." || value.contains('/') || value.contains('\\') {
        bail!("变量 '{}' 不能包含路径分隔符或特殊目录名", name);
    }

    Ok(())
}

fn is_path_like_variable(name: &str) -> bool {
    matches!(name, "project-name" | "project_name")
        || name.ends_with("-name")
        || name.ends_with("_name")
}

fn template_variable_names(meta: &TemplateMeta, provided: &HashMap<String, String>) -> Vec<String> {
    let mut names = HashSet::new();

    if let Some(_variables) = &meta.variables {
        names.extend(provided.keys().cloned());
    }

    names.extend(provided.keys().cloned());

    names
        .into_iter()
        .map(|name| to_original_key(&name))
        .collect()
}

fn insert_aliases(context: &mut TeraContext, name: &str, value: &str) {
    let name = name.to_string();
    let value = value.to_string();

    context.insert(name.clone(), &value.clone());
    context.insert(to_safe_key(&name), &value.clone());
    context.insert(to_original_key(&name), &value);
}

/// 插入模板变量，并写入 `project-name` / `project_name` 这类等价别名
///
/// 当不同来源为同一个规范化变量提供了不同值时返回错误
pub fn insert_variable(
    variables: &mut HashMap<String, String>,
    key: String,
    value: String,
    source: &str,
) -> Result<()> {
    let canonical = canonical_key(&key);

    for existing_key in alias_keys(&key) {
        if let Some(existing_value) = variables.get(&existing_key)
            && existing_value != &value
        {
            bail!(
                "变量 '{}' 的值冲突：已有值 '{}', {} 提供了 '{}'",
                canonical,
                existing_value,
                source,
                value
            );
        }
    }

    for alias in alias_keys(&key) {
        variables.insert(alias, value.clone());
    }

    Ok(())
}

pub fn normalize_variables(
    input: HashMap<String, String>,
    source: &str,
) -> Result<HashMap<String, String>> {
    let mut variables = HashMap::new();

    for (key, value) in input {
        insert_variable(&mut variables, key, value, source)?;
    }

    Ok(variables)
}

pub fn variable_exists(variable: &HashMap<String, String>, key: &str) -> bool {
    alias_keys(key)
        .iter()
        .any(|alias| variable.contains_key(alias))
}

pub fn alias_keys(key: &str) -> Vec<String> {
    let canonical = canonical_key(key);
    let safe = canonical.replace('-', "_");

    if canonical == safe {
        vec![canonical]
    } else {
        vec![canonical, safe]
    }
}

pub fn canonical_key(key: &str) -> String {
    key.replace('-', "_")
}

fn to_safe_key(name: &str) -> String {
    name.replace('-', "_")
}

fn to_original_key(name: &str) -> String {
    name.replace('_', "-")
}
