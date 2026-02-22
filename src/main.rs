use dialoguer::{theme::ColorfulTheme, Select};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::Path;
use colored::*; // 色付き出力用

struct Preset {
    name: &'static str,
    color: &'static str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let presets = vec![
        Preset { name: "Angular Red", color: "#DD0031" },
        Preset { name: "Azure Blue", color: "#007FFF" },
        Preset { name: "JavaScript Yellow", color: "#F7DF1E" },
        Preset { name: "Node JS Green", color: "#68A063" },
        Preset { name: "React Blue", color: "#61DAFB" },
        Preset { name: "Svelte Orange", color: "#FF3E00" },
        Preset { name: "Vue Green", color: "#42B883" },
        Preset { name: "Rust Orange", color: "#DEA584" },
        Preset { name: "Reset (Clear Settings)", color: "reset" },
    ];

    let args: Vec<String> = env::args().collect();

    let selected_color = if args.len() > 1 {
        args[1].clone()
    } else {
        // 表示用のラベル作成 (色付きの四角 ■ を追加)
        let items: Vec<String> = presets.iter().map(|p| {
            if p.color == "reset" {
                format!("  {}", p.name)
            } else {
                let (r, g, b) = hex_to_rgb(p.color);
                // ターミナルにRGBで色付きの■を表示
                format!("{} {}", "■".truecolor(r, g, b), p.name)
            }
        }).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Cursorの色を選んでください")
            .default(0)
            .items(&items)
            .interact_opt()?;

        match selection {
            Some(index) => presets[index].color.to_string(),
            None => return Ok(()),
        }
    };

    apply_color(&selected_color)
}

// Hex (#RRGGBB) を (u8, u8, u8) に変換するヘルパー
fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}

fn apply_color(color: &str) -> Result<(), Box<dyn std::error::Error>> {
    let vscode_dir = Path::new(".vscode");
    let settings_path = vscode_dir.join("settings.json");

    if !vscode_dir.exists() {
        fs::create_dir(vscode_dir)?;
    }

    let mut settings: Value = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content).unwrap_or(json!({}))
    } else {
        json!({})
    };

    if color == "reset" {
        if let Some(map) = settings.as_object_mut() {
            map.remove("workbench.colorCustomizations");
        }
        println!("{}", "設定をリセットしました。".yellow());
    } else {
        let (r, g, b) = hex_to_rgb(color);
        let customizations = json!({
            "activityBar.background": color,
            "activityBar.activeBackground": color,
            "activityBar.foreground": "#e7e7e7",
            "statusBar.background": color,
            "statusBar.foreground": "#e7e7e7",
            "titleBar.activeBackground": color,
            "titleBar.activeForeground": "#e7e7e7",
        });
        settings["workbench.colorCustomizations"] = customizations;
        println!("{} を {} に適用しました。", "■".truecolor(r, g, b), color.bold());
    }

    let pretty_json = serde_json::to_string_pretty(&settings)?;
    fs::write(settings_path, pretty_json)?;

    Ok(())
}
