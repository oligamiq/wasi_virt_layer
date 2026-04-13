use std::fs;

use camino::Utf8PathBuf;
use eyre::Context as _;
use toml_edit::{Document, DocumentMut, Item};

/// Represents a single modification to a `Cargo.toml` file, storing both the original
/// string and the applied changes so that it can be cleanly reverted later.
#[derive(Debug, Clone)]
pub struct TomlRestorer {
    path: Utf8PathBuf,
    original: String,
    changed: String,
}

use std::sync::{Arc, Mutex};

/// A thread-safe collection of `TomlRestorer` instances.
///
/// Automatically drops and reverts any `Cargo.toml` modifications if a thread panics.
#[derive(Debug, Clone)]
pub struct TomlRestorers {
    inner: Arc<Mutex<Vec<TomlRestorer>>>,
}

impl Drop for TomlRestorers {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.restore_if_needed();
        }
    }
}

impl TomlRestorers {
    /// Creates a new, empty collection of `TomlRestorer` instances.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates a new `TomlRestorers` collection initialized with a single restorer.
    pub fn with(restorer: TomlRestorer) -> Self {
        Self {
            inner: Arc::new(Mutex::new(vec![restorer])),
        }
    }

    /// Extends the collection with multiple restorers.
    pub fn extend(&mut self, restorers: Vec<TomlRestorer>) {
        for restorer in restorers {
            self.push(restorer);
        }
    }

    /// Adds a single restorer to the collection.
    pub fn push(&mut self, restorer: TomlRestorer) {
        self.inner.lock().unwrap().push(restorer);
    }

    /// Restore if any restorers are present. This does not consume the restorers, but clears them after restoring.
    pub fn restore_if_needed(&self) {
        if let Ok(mut restorers) = self.inner.lock() {
            if !restorers.is_empty() {
                for restorer in restorers.iter() {
                    let _ = std::fs::write(&restorer.path, &restorer.original);
                }
                restorers.clear();
            }
        }
    }

    /// Explicitly restores all recorded modifications and consumes the collection.
    pub fn restore(self) -> eyre::Result<()> {
        let mut restorers_lock = self
            .inner
            .lock()
            .map_err(|_| eyre::eyre!("Failed to lock restorers"))?;
        let restorers = std::mem::take(&mut *restorers_lock);
        drop(restorers_lock); // Release lock before I/O
        for restorer in restorers {
            restorer.restore()?;
        }
        Ok(())
    }
}

impl TomlRestorer {
    /// Creates a new `TomlRestorer` recording the path, original content, and changed content.
    pub fn new(path: &Utf8PathBuf, original: String, changed: String) -> Self {
        Self {
            path: path.clone(),
            original,
            changed,
        }
    }

    /// Reads the original state, writes new content, and returns a restorer for later reversion.
    pub fn with_write(path: &Utf8PathBuf, changed: String) -> eyre::Result<Self> {
        let file_data = fs::read_to_string(&path).wrap_err("Failed to read manifest file")?;
        fs::write(&path, &changed).wrap_err("Failed to write manifest file")?;
        Ok(Self::new(path, file_data, changed))
    }

    /// Consumes the restorer by pushing it into a list and merging duplicates where possible.
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

    /// Merges this restorer with another, reducing redundant entries for the same file.
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

    /// Reverts the file content to its original state recorded by this restorer.
    pub fn restore(self) -> eyre::Result<()> {
        fs::write(&self.path, self.original).wrap_err("Failed to write manifest file")?;
        Ok(())
    }
}

/// Indicates whether a feature is present and where it is enabled.
pub enum HasFeature {
    /// The feature is disabled.
    Disabled,
    /// The feature is enabled directly in the crate's `Cargo.toml`.
    EnabledOnNormal,
    /// The feature is inherited from the `[workspace.dependencies]` table.
    EnabledOnWorkspace,
}

/// Verifies and manipulates Cargo manifest features for a specific crate.
///
/// Handles both the crate's individual `Cargo.toml` and the root workspace
/// `Cargo.toml` to accurately determine and adjust feature sets.
#[derive(Debug)]
pub struct FeatureChecker<'a, 'b, 'c, 'd> {
    feature: Option<&'a str>,
    manifest_path: &'b Utf8PathBuf,
    root_manifest_path: &'c Utf8PathBuf,
    crate_name: &'d str,
}

impl<'a, 'b, 'c, 'd> FeatureChecker<'a, 'b, 'c, 'd> {
    /// Initializes a new `FeatureChecker` targeting a specific feature across manifest boundaries.
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

    /// Initializes a `FeatureChecker` without a specific feature target for general manifest checks.
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
        if item.is_none() {
            return false;
        }
        match item {
            Item::Table(table) => table.get("features").map_or(false, |v| {
                v.as_array()
                    .map(|arr| arr.iter().any(|s| s.as_str() == Some(feature)))
                    .unwrap_or(false)
            }),
            Item::Value(toml_edit::Value::InlineTable(inline)) => {
                inline.get("features").map_or(false, |v| {
                    v.as_array()
                        .map(|arr| arr.iter().any(|s| s.as_str() == Some(feature)))
                        .unwrap_or(false)
                })
            }
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

    /// Investigates the current state of a feature within the manifest or workspace dependencies.
    pub fn has(&self) -> eyre::Result<HasFeature> {
        let Self {
            crate_name,
            feature,
            ..
        } = self;

        let feature = feature.ok_or_else(|| eyre::eyre!("Feature is not set"))?;

        let doc = self.read_manifest::<Document<String>>()?;

        if doc
            .get("dependencies")
            .map_or(true, |inner| inner.is_none())
        {
            eyre::bail!("No dependencies found in manifest");
        }

        if doc["dependencies"]
            .get(crate_name)
            .map_or(true, |inner| inner.is_none())
        {
            eyre::bail!("Required crate `{crate_name}` not found in dependencies");
        }

        let crate_setting = &doc["dependencies"][crate_name];

        // check normal crate setting
        Ok(if Self::has_feature(crate_setting, feature) {
            HasFeature::EnabledOnNormal
        } else {
            // check workspace
            match &crate_setting.get("workspace") {
                Some(v) if v.as_bool().unwrap_or(false) => {
                    let doc = self.read_workspace_manifest::<Document<String>>()?;

                    let crate_setting = &doc["workspace"]["dependencies"][crate_name];

                    if Self::has_feature(crate_setting, feature) {
                        HasFeature::EnabledOnWorkspace
                    } else {
                        HasFeature::Disabled
                    }
                }
                None => HasFeature::Disabled,
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

    /// Dynamically enabling or disabling a specific feature and returns a restorer if changed.
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
