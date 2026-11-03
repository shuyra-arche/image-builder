use std::path::{Path, PathBuf};
use std::process::Command;

use unit::ImageBuild;

use crate::{checked_elf, contains, package_id};

pub(crate) struct BuildGroup<'a> {
    manifest: &'static str,
    package: &'static str,
    target: &'static str,
    profile: &'static str,
    features: &'static [&'static str],
    default_features: bool,
    builds: Vec<&'a ImageBuild>,
}

pub(crate) fn groups<'a>(picked: &[&'a ImageBuild]) -> Vec<BuildGroup<'a>> {
    let mut groups: Vec<BuildGroup<'a>> = Vec::new();
    for build in picked {
        let existing = groups.iter().position(|group| {
            group.manifest == build.manifest
                && group.package == build.package
                && group.target == build.target
                && group.profile == build.profile
                && group.features == build.features
                && group.default_features == build.default_features
        });
        match existing {
            Some(index) => groups[index].builds.push(build),
            None => groups.push(BuildGroup {
                manifest: build.manifest,
                package: build.package,
                target: build.target,
                profile: build.profile,
                features: build.features,
                default_features: build.default_features,
                builds: vec![build],
            }),
        }
    }
    groups
}

pub(crate) fn build_group(
    root: &Path,
    work: &Path,
    group: &BuildGroup<'_>,
) -> Result<Vec<(&'static str, PathBuf)>, String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let manifest = root.join(group.manifest);
    let expected = package_id(&cargo, &manifest, group.package)?;
    let mut args = vec![
        "build".to_string(),
        "--locked".to_string(),
        "--manifest-path".to_string(),
        manifest.to_string_lossy().into_owned(),
        "-p".to_string(),
        group.package.to_string(),
        "--message-format=json".to_string(),
        "--target".to_string(),
        group.target.to_string(),
        "--target-dir".to_string(),
        work.to_string_lossy().into_owned(),
        "--profile".to_string(),
        group.profile.to_string(),
    ];
    let mut selected_bins: Vec<&'static str> = Vec::new();
    for build in &group.builds {
        if !selected_bins.contains(&build.bin) {
            selected_bins.push(build.bin);
            args.push("--bin".to_string());
            args.push(build.bin.to_string());
        }
    }
    if !group.features.is_empty() {
        args.push("--features".to_string());
        args.push(group.features.join(","));
    }
    if !group.default_features {
        args.push("--no-default-features".to_string());
    }
    let output = Command::new(&cargo)
        .args(&args)
        .current_dir(root)
        .output()
        .map_err(|e| format!("起不动嵌套 cargo：{e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let tail: String = stderr.chars().rev().take(4000).collect::<String>().chars().rev().collect();
        return Err(format!(
            "{} 编不过（{}）：\n{tail}",
            group.package,
            output.status
        ));
    }

    let mut found = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let line = line.trim();
        if !line.starts_with('{') {
            continue;
        }
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if value["reason"] != "compiler-artifact"
            || value["package_id"] != expected.as_str()
            || !contains(&value["target"]["kind"], "bin")
            || value["profile"]["test"].as_bool().unwrap_or(true)
        {
            continue;
        }
        let matching: Vec<&ImageBuild> = group
            .builds
            .iter()
            .copied()
            .filter(|build| value["target"]["name"] == build.bin)
            .collect();
        if matching.is_empty() {
            continue;
        }
        let Some(executable) = value["executable"].as_str() else {
            return Err(format!("镜像 {} 的可执行产物缺失", matching[0].image));
        };
        // Distinct logical image names may share one Cargo coordinate; the same
        // resolved executable belongs to every matching image, never guessed.
        for build in matching {
            println!(
                "image resolved: image={} package={} bin={} target={} profile={} features={:?} executable={}",
                build.image,
                value["package_id"].as_str().unwrap_or(""),
                build.bin,
                group.target,
                group.profile,
                value["features"],
                executable
            );
            found.push((build.image, PathBuf::from(executable)));
        }
    }

    let mut artifacts = Vec::new();
    for build in &group.builds {
        let matches: Vec<_> = found
            .iter()
            .filter(|(image, _)| *image == build.image)
            .collect();
        match matches.as_slice() {
            [] => return Err(format!("Cargo 缺少镜像 {} 的执行产物", build.image)),
            [single] => artifacts.push((build.image, checked_elf(&single.1)?)),
            _ => return Err(format!("Cargo 对镜像 {} 报出多个执行产物", build.image)),
        }
    }
    Ok(artifacts)
}
