use anyhow::{Result, bail};
use colored::Colorize;
use nebula_core::template::{
    self, initialize_project, insert_variable, load_template_meta, variable_exists,
};
use nebula_protos::{InitOptions, OverwriteMode, TemplateMeta};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn list_templates() -> Result<()> {
    let templates = template::list_template()?;

    println!("{}", "可用模板：".bold().green());
    for summary in templates {
        match (summary.version, summary.description) {
            (Some(version), Some(description)) => {
                println!(" {} {} {}", summary.name.bold(), version.dimmed(), description);
            },
            _ => println!(" {} {}", summary.name.bold(), "（无有效元数据）".dimmed()),
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn init_template(
    template_name: String,
    project_name: String,
    output: PathBuf,
    vars: Vec<(String, String)>,
    author: Option<String>,
    non_interactive: bool,
    force: bool,
    dry_run: bool,
) -> Result<()> {
    let meta = load_template_meta(&template_name)?;
    let mut variables = HashMap::new();

    insert_variable(&mut variables, "project-name".to_string(), project_name, "--name")?;

    for (key, value) in vars {
        insert_variable(&mut variables, key, value, "--var")?;
    }

    if let Some(author) = author {
        insert_variable(&mut variables, "author".to_string(), author, "--author")?;
    }

    if !non_interactive {
        fill_missing_required_variables(&meta, &mut variables)?;
    }

    println!("{} 从模板 '{}' 初始化项目...", "🔄️".bold(), template_name.bold());

    let report = initialize_project(InitOptions {
        template_name,
        output_base: output.clone(),
        variables,
        overwrite: if force {
            OverwriteMode::Overwrite
        } else {
            OverwriteMode::Fail
        },
        dry_run,
    })?;

    if dry_run {
        println!("{} 以下是将创建的路径：", "🔎".bold());
    }

    for dir in &report.created_dirs {
        println!("{} {}", "📁".bold(), dir.display());
    }
    for file in &report.created_files {
        println!("{} {}", "📝".green(), file.display());
    }
    for file in &report.skipped_files {
        println!("{} {}", "⏩".yellow(), file.display());
    }

    if dry_run {
        println!("{} dry-run 完成，未写入文件。", "✅".bold().green());
    } else {
        println!("{} 项目生成成功！输出父目录：{}", "✅".bold().green(), output.display());
    }

    Ok(())
}

fn fill_missing_required_variables(
    meta: &TemplateMeta,
    variables: &mut HashMap<String, String>,
) -> Result<()> {
    let Some(definitions) = &meta.variables else {
        return Ok(());
    };

    let mut names: Vec<_> = definitions.keys().cloned().collect();
    names.sort();

    for name in names {
        if variable_exists(variables, &name) {
            continue;
        }

        let Some(definition) = definitions.get(&name) else {
            continue;
        };

        if definition
            .default
            .as_deref()
            .is_some_and(|default| !default.is_empty())
        {
            continue;
        }

        let prompt_text = definition.prompt.as_deref().unwrap_or(&name);
        let value = prompt_for(&name, prompt_text)?;
        insert_variable(variables, name, value, "prompt")?;
    }

    Ok(())
}

fn prompt_for(var_name: &str, prompt_text: &str) -> Result<String> {
    print!("{}: ", prompt_text);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let value = input.trim().to_string();

    if value.is_empty() {
        bail!("变量 '{}' 不能为空", var_name);
    }

    Ok(value)
}

pub fn parse_key_value(input: &str) -> Result<(String, String), String> {
    let (key, value) = input
        .split_once('=')
        .ok_or_else(|| "变量格式必须为 key=value".to_string())?;

    let key = key.trim();
    if key.is_empty() {
        return Err("变量名不能为空".to_string());
    }

    Ok((key.to_string(), value.to_string()))
}
