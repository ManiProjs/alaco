use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use miette::Result;

#[derive(Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub entry: PathBuf,
    pub dependencies: BTreeMap<String, String>,
    pub build: BuildConfig,
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub target: String,
    pub backend: String,
    pub compiler: String,
    pub flags: Vec<String>,
    pub cache: bool,
    pub source_map: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: "native".to_owned(),
            backend: "cpp".to_owned(),
            compiler: "clang++".to_owned(),
            flags: Vec::new(),
            cache: true,
            source_map: false,
        }
    }
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path).map_err(|error| {
            miette::miette!("cannot read package manifest '{}': {error}", path.display())
        })?;
        Self::parse(&source, path)
    }

    fn parse(source: &str, path: &Path) -> Result<Self> {
        let document: toml::Value = toml::from_str(source).map_err(|error| {
            miette::miette!("invalid package manifest '{}': {error}", path.display())
        })?;
        let package = document
            .get("package")
            .and_then(toml::Value::as_table)
            .ok_or_else(|| miette::miette!("manifest must contain a [package] table"))?;

        let name = required_string(package, "name")?;
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(miette::miette!(
                "invalid package name '{name}'; use ASCII letters, digits, '-' or '_'"
            ));
        }

        let version = required_string(package, "version")?;
        if version.split('.').count() < 3
            || !version.split('.').take(3).all(|part| {
                let numeric = part.split(['-', '+']).next().unwrap_or_default();
                !numeric.is_empty() && numeric.chars().all(|c| c.is_ascii_digit())
            })
        {
            return Err(miette::miette!(
                "invalid package version '{version}'; expected a semantic version such as 0.1.0"
            ));
        }

        let edition = package
            .get("edition")
            .and_then(toml::Value::as_str)
            .unwrap_or("2026")
            .to_owned();
        if edition != "2026" {
            return Err(miette::miette!(
                "unsupported Alaco edition '{edition}'; currently supported edition is '2026'"
            ));
        }

        let entry = package
            .get("entry")
            .and_then(toml::Value::as_str)
            .unwrap_or("src/main.aco");
        let entry = PathBuf::from(entry);
        if entry.is_absolute()
            || entry
                .components()
                .any(|component| component == std::path::Component::ParentDir)
        {
            return Err(miette::miette!(
                "package entry '{}' must be a relative path inside the package",
                entry.display()
            ));
        }
        if entry.extension().and_then(|ext| ext.to_str()) != Some("aco") {
            return Err(miette::miette!("package entry must point to an .aco file"));
        }

        let mut dependencies = BTreeMap::new();
        if let Some(deps) = document.get("dependencies") {
            let deps = deps.as_table().ok_or_else(|| {
                miette::miette!("[dependencies] must be a table of dependency declarations")
            })?;
            for (dependency_name, declaration) in deps {
                if dependency_name.is_empty()
                    || !dependency_name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                {
                    return Err(miette::miette!(
                        "invalid dependency name '{dependency_name}'"
                    ));
                }
                let description = if let Some(version) = declaration.as_str() {
                    version.to_owned()
                } else if let Some(table) = declaration.as_table() {
                    let local_path = table.get("path").and_then(toml::Value::as_str);
                    let version = table.get("version").and_then(toml::Value::as_str);
                    if let Some(local_path) = local_path {
                        format!("path = \"{local_path}\"")
                    } else if let Some(version) = version {
                        version.to_owned()
                    } else {
                        return Err(miette::miette!(
                            "dependency '{dependency_name}' must specify a version string or a 'path'/'version' field"
                        ));
                    }
                } else {
                    return Err(miette::miette!(
                        "dependency '{dependency_name}' must be a version string or an inline table"
                    ));
                };
                dependencies.insert(dependency_name.clone(), description);
            }
        }

        let mut build = BuildConfig::default();
        if let Some(value) = document.get("build") {
            let table = value
                .as_table()
                .ok_or_else(|| miette::miette!("[build] must be a table"))?;
            if let Some(target) = table.get("target").and_then(toml::Value::as_str) {
                build.target = target.to_owned();
            }
            if let Some(backend) = table.get("backend").and_then(toml::Value::as_str) {
                build.backend = backend.to_owned();
            }
            if let Some(compiler) = table.get("compiler").and_then(toml::Value::as_str) {
                build.compiler = compiler.to_owned();
            }
            if let Some(flags) = table.get("flags") {
                build.flags = flags
                    .as_array()
                    .ok_or_else(|| miette::miette!("[build].flags must be an array of strings"))?
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .map(str::to_owned)
                            .ok_or_else(|| miette::miette!("[build].flags entries must be strings"))
                    })
                    .collect::<Result<Vec<_>>>()?;
            }
            if let Some(cache) = table.get("cache") {
                build.cache = cache
                    .as_bool()
                    .ok_or_else(|| miette::miette!("[build].cache must be a boolean"))?;
            }
            if let Some(source_map) = table.get("source-map") {
                build.source_map = source_map
                    .as_bool()
                    .ok_or_else(|| miette::miette!("[build].source-map must be a boolean"))?;
            }
        }
        if build.target != "native"
            && (build.target.is_empty()
                || !build
                    .target
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')))
        {
            return Err(miette::miette!(
                "invalid build target '{}'; use 'native' or a compiler target triple",
                build.target
            ));
        }
        if build.backend != "cpp" {
            return Err(miette::miette!(
                "unsupported build backend '{}'; currently supported backend is 'cpp'",
                build.backend
            ));
        }
        if build.compiler.trim().is_empty() {
            return Err(miette::miette!("[build].compiler must not be empty"));
        }

        Ok(Self {
            name,
            version,
            edition,
            entry,
            dependencies,
            build,
        })
    }
}

fn required_string(table: &toml::map::Map<String, toml::Value>, key: &str) -> Result<String> {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| miette::miette!("[package] must define a non-empty string '{key}'"))
}

#[cfg(test)]
mod tests {
    use super::Manifest;
    use std::path::Path;

    #[test]
    fn parses_package_metadata_entry_and_dependencies() {
        let manifest = Manifest::parse(
            "[package]\nname='demo'\nversion='1.2.3'\nentry='app/main.aco'\n[dependencies]\ncolors='^1.0'\nlocal={ path='../local' }\n",
            Path::new("Alaco.toml"),
        ).unwrap();
        assert_eq!(manifest.name, "demo");
        assert_eq!(manifest.version, "1.2.3");
        assert_eq!(manifest.entry, Path::new("app/main.aco"));
        assert_eq!(manifest.dependencies.len(), 2);
        assert_eq!(manifest.dependencies["colors"], "^1.0");
        assert_eq!(manifest.dependencies["local"], "path = \"../local\"");
    }

    #[test]
    fn rejects_unsafe_entry_and_invalid_versions() {
        let unsafe_entry = Manifest::parse(
            "[package]\nname='demo'\nversion='1.0.0'\nentry='../main.aco'\n",
            Path::new("Alaco.toml"),
        );
        assert!(unsafe_entry.is_err());

        let invalid_version = Manifest::parse(
            "[package]\nname='demo'\nversion='next'\n",
            Path::new("Alaco.toml"),
        );
        assert!(invalid_version.is_err());
    }

    #[test]
    fn parses_backward_compatible_build_configuration() {
        let manifest = Manifest::parse("[package]\nname='demo'\nversion='1.0.0'\n[build]\ncompiler='clang++'\nflags=['-O2']\ncache=false\nsource-map=true\n", Path::new("Alaco.toml")).unwrap();
        assert_eq!(manifest.build.target, "native");
        assert_eq!(manifest.build.flags, vec!["-O2"]);
        assert!(!manifest.build.cache);
        assert!(manifest.build.source_map);
    }

    #[test]
    fn rejects_invalid_build_target_values() {
        let manifest = Manifest::parse(
            "[package]\nname='demo'\nversion='1.0.0'\n[build]\ntarget=''\n",
            Path::new("Alaco.toml"),
        );
        assert!(manifest.is_err());

        let manifest = Manifest::parse(
            "[package]\nname='demo'\nversion='1.0.0'\n[build]\ntarget='native; touch bad'\n",
            Path::new("Alaco.toml"),
        );
        assert!(manifest.is_err());
    }
}
