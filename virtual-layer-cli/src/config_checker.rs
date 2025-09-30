use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;
use toml_edit::{Document, DocumentMut, Item};

#[derive(Debug, Clone)]
pub struct TomlRestorer {
    path: Utf8PathBuf,
    original: String,
    changed: String,
}

#[derive(Debug, Clone)]
pub struct TomlRestorers(Vec<TomlRestorer>);

impl TomlRestorers {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with(restorer: TomlRestorer) -> Self {
        Self(vec![restorer])
    }

    pub fn extend(&mut self, restorers: Vec<TomlRestorer>) {
        for restorer in restorers {
            self.push(restorer);
        }
    }

    pub fn push(&mut self, restorer: TomlRestorer) {
        self.0.push(restorer);
    }

    pub fn restore(self) -> eyre::Result<()> {
        for restorer in self.0 {
            restorer.restore()?;
        }
        Ok(())
    }
}

impl TomlRestorer {
    pub fn new(path: &Utf8PathBuf, original: String, changed: String) -> Self {
        Self {
            path: path.clone(),
            original,
            changed,
        }
    }

    pub fn with_write(path: &Utf8PathBuf, changed: String) -> eyre::Result<Self> {
        let file_data = fs::read_to_string(&path).wrap_err("Failed to read manifest file")?;
        fs::write(&path, &changed).wrap_err("Failed to write manifest file")?;
        Ok(Self::new(path, file_data, changed))
    }

    pub fn push(self, mut restorers: Vec<TomlRestorer>) -> Vec<TomlRestorer> {
        loop {
            let mut merged = vec![];
            let len = restorers.len();
            for restorer in restorers {
                merged.extend(self.merge(restorer));
            }
            if merged.len() == len {
                break merged;
            }
            restorers = merged;
        }
    }

    pub fn merge(&self, other: TomlRestorer) -> Vec<TomlRestorer> {
        if self.path != other.path {
            return vec![self.clone(), other];
        }

        if other.original == self.changed {
            vec![TomlRestorer::new(
                &self.path,
                self.original.clone(),
                other.changed,
            )]
        } else if self.original == other.changed {
            vec![TomlRestorer::new(
                &self.path,
                other.original,
                self.changed.clone(),
            )]
        } else if self.original == other.original {
            panic!("Merging two same original toml restorer");
        } else {
            vec![self.clone(), other]
        }
    }

    pub fn restore(self) -> eyre::Result<()> {
        fs::write(&self.path, self.original).wrap_err("Failed to write manifest file")?;
        Ok(())
    }
}

pub enum HasFeature {
    Disabled,
    EnabledOnNormal,
    EnabledOnWorkspace,
}

#[derive(Debug)]
pub struct FeatureChecker<'a, 'b, 'c, 'd> {
    feature: Option<&'a str>,
    manifest_path: &'b Utf8PathBuf,
    root_manifest_path: &'c Utf8PathBuf,
    crate_name: &'d str,
}

impl<'a, 'b, 'c, 'd> FeatureChecker<'a, 'b, 'c, 'd> {
    pub const fn new(
        feature: &'a str,
        manifest_path: &'b Utf8PathBuf,
        root_manifest_path: &'c Utf8PathBuf,
        crate_name: &'d str,
    ) -> Self {
        Self {
            feature: Some(feature),
            manifest_path,
            root_manifest_path,
            crate_name,
        }
    }

    pub const fn new_no_feature(
        manifest_path: &'b Utf8PathBuf,
        root_manifest_path: &'c Utf8PathBuf,
        crate_name: &'d str,
    ) -> Self {
        Self {
            manifest_path,
            feature: None,
            root_manifest_path,
            crate_name,
        }
    }

    fn has_feature(item: &Item, feature: &str) -> bool {
        match item {
            Item::Table(table) => table["features"]
                .as_array()
                .map(|arr| arr.iter().any(|s| s.as_str() == Some(feature)))
                .unwrap_or(false),
            _ => false,
        }
    }

    fn read_manifest<T: std::str::FromStr<Err = toml_edit::TomlError>>(&self) -> eyre::Result<T> {
        let file_data =
            fs::read_to_string(&self.manifest_path).wrap_err("Failed to read manifest file")?;
        let doc = file_data
            .parse::<T>()
            .wrap_err("Failed to parse manifest file")?;
        Ok(doc)
    }

    fn read_workspace_manifest<T: std::str::FromStr<Err = toml_edit::TomlError>>(
        &self,
    ) -> eyre::Result<T> {
        let file_data = fs::read_to_string(&self.root_manifest_path)
            .wrap_err("Failed to read workspace manifest file")?;
        let doc = file_data
            .parse::<T>()
            .wrap_err("Failed to parse workspace manifest file")?;
        Ok(doc)
    }

    pub fn has(&self) -> eyre::Result<HasFeature> {
        let Self {
            crate_name,
            feature,
            ..
        } = self;

        let feature = feature.ok_or_else(|| eyre::eyre!("Feature is not set"))?;

        let doc = self.read_manifest::<Document<String>>()?;
        let crate_setting = &doc["dependencies"][crate_name];

        if matches!(crate_setting, Item::None) {
            eyre::bail!("Crate `{crate_name}` not found in dependencies");
        }

        // check normal crate setting
        Ok(if Self::has_feature(crate_setting, feature) {
            HasFeature::EnabledOnNormal
        } else {
            // check workspace
            match &crate_setting["workspace"] {
                v if v.as_bool().unwrap_or(false) => {
                    let doc = self.read_workspace_manifest::<Document<String>>()?;

                    let crate_setting = &doc["workspace"]["dependencies"][crate_name];

                    if Self::has_feature(crate_setting, feature) {
                        HasFeature::EnabledOnWorkspace
                    } else {
                        HasFeature::Disabled
                    }
                }
                _ => HasFeature::Disabled,
            }
        })
    }

    fn set_table(table: &mut Item, feature: &str, on: bool) -> eyre::Result<()> {
        if on {
            if matches!(table.get("features"), None) {
                table["features"] = toml_edit::value(toml_edit::Array::new());
            }
            if table["features"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|s| s.as_str())
                .any(|s| s == feature)
            {
                return Ok(());
            }
            table["features"].as_array_mut().unwrap().push(feature);
        } else {
            if matches!(table.get("features"), None) {
                return Ok(());
            }
            if table["features"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|s| s.as_str())
                .any(|s| s == feature)
            {
                table["features"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|s| s.as_str() != Some(feature));
                if table["features"].as_array().unwrap().is_empty() {
                    table["features"] = Item::None;
                }
            }
        }

        Ok(())
    }

    pub fn set(&self, on: bool) -> eyre::Result<Option<TomlRestorer>> {
        let Self {
            feature,
            manifest_path,
            root_manifest_path,
            crate_name,
        } = self;

        let feature = feature.ok_or_else(|| eyre::eyre!("Feature is not set"))?;

        let now = self.has()?;
        let mut doc = self.read_manifest::<DocumentMut>()?;

        let crate_setting = &mut doc["dependencies"][crate_name];

        let (path, data) = match (now, on) {
            (HasFeature::Disabled, true) | (HasFeature::EnabledOnNormal, false) => {
                Self::set_table(crate_setting, feature, on)?;
                (manifest_path.to_owned(), doc)
            }
            (HasFeature::EnabledOnWorkspace, false) => {
                log::warn!(
                    "Feature `{feature}` is enabled on workspace, so changing it may affect other crates."
                );

                let mut doc = self.read_workspace_manifest::<DocumentMut>()?;
                let crate_setting = &mut doc["workspace"]["dependencies"][crate_name];

                Self::set_table(crate_setting, feature, on)?;

                (root_manifest_path.to_owned(), doc)
            }
            _ => {
                return Ok(None);
            }
        };

        Ok(Some(TomlRestorer::with_write(&path, data.to_string())?))
    }

    /// Set [profile.release] debug = true/false
    /// [profile.release]
    /// debug = true
    pub fn set_dwarf(&self, on: bool) -> eyre::Result<TomlRestorer> {
        // if workspace, we set workspace
        // else we set normal toml
        fn set(doc: &mut DocumentMut, on: bool) -> eyre::Result<()> {
            if let Some(debug) = doc.get_mut("profile.release") {
                if debug.as_bool().unwrap_or(false) == on {
                    return Ok(());
                }
                debug["debug"] = toml_edit::value(on);
            }

            if let Some(profile) = doc.get_mut("profile") {
                if let Some(release) = profile.get_mut("release") {
                    release["debug"] = toml_edit::value(on);
                } else {
                    profile["release"] = toml_edit::table();
                    profile["release"]["debug"] = toml_edit::value(on);
                }
            } else {
                // inline
                let mut profile = toml_edit::Table::new();
                profile.set_implicit(true);
                profile["release"] = toml_edit::table();
                profile["release"]["debug"] = toml_edit::value(on);
                doc["profile"] = toml_edit::Item::Table(profile);
            }

            Ok(())
        }

        if self.manifest_path != self.root_manifest_path {
            let mut doc = self.read_workspace_manifest::<DocumentMut>()?;
            set(&mut doc, on)?;

            return TomlRestorer::with_write(&self.root_manifest_path, doc.to_string());
        }

        let mut doc = self.read_manifest::<DocumentMut>()?;
        set(&mut doc, on)?;
        return TomlRestorer::with_write(&self.manifest_path, doc.to_string());
    }
}
