use crate::commit::{CommitConfig, CommitType};
use crate::repository::Repository;
use crate::CommitsMetadata;
use anyhow::Result;
use config::{Config, File};
use std::collections::HashMap;
use std::path::PathBuf;

type CommitsMetadataSettings = HashMap<String, CommitConfig>;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Settings {
    pub changelog_path: Option<PathBuf>,
    #[serde(default)]
    pub hooks: Vec<String>,
    #[serde(default)]
    pub commit_types: CommitsMetadataSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            changelog_path: Some(PathBuf::from("CHANGELOG.md")),
            commit_types: Default::default(),
            hooks: vec![],
        }
    }
}

impl Settings {
    pub(crate) fn get(repository: &Repository) -> Result<Self> {
        match repository.get_repo_dir() {
            Some(path) => {
                if path.exists() {
                    let mut s = Config::new();
                    s.merge(File::from(path.join("coco.toml")))?;
                    s.try_into()
                        .map_err(|err| anyhow!("Config format error : {}", err))
                } else {
                    Err(anyhow!(
                        "Missing `coco.toml` config file in {}",
                        path.display()
                    ))
                }
            }
            None => Err(anyhow!("Current dir is not a git repository")),
        }
    }

    pub(crate) fn commit_types(&self) -> CommitsMetadata {
        let commit_settings = self.commit_types.clone();
        let mut custom_types = HashMap::new();

        commit_settings.iter().for_each(|(key, value)| {
            let _ = custom_types.insert(CommitType::from(key.as_str()), value.clone());
        });

        let mut default_types = Settings::get_default_commit_config();

        default_types.extend(custom_types);

        default_types
    }

    fn get_default_commit_config() -> CommitsMetadata {
        let mut default_types = HashMap::new();
        default_types.insert(
            CommitType::Feature,
            CommitConfig::new("Features", "create a `feature` commit"),
        );
        default_types.insert(
            CommitType::BugFix,
            CommitConfig::new("Bug Fixes", "create a `bug fix` commit"),
        );
        default_types.insert(
            CommitType::Chore,
            CommitConfig::new("Miscellaneous Chores", "create a `chore` commit"),
        );
        default_types.insert(
            CommitType::Revert,
            CommitConfig::new("Revert", "create a `revert` commit"),
        );
        default_types.insert(
            CommitType::Performances,
            CommitConfig::new("Performance Improvements", "create a `performance` commit"),
        );
        default_types.insert(
            CommitType::Documentation,
            CommitConfig::new("Documentation", "create a `documentation` commit"),
        );
        default_types.insert(
            CommitType::Style,
            CommitConfig::new("Style", "create a `style` commit"),
        );
        default_types.insert(
            CommitType::Refactoring,
            CommitConfig::new("Refactoring", "create a `refactor` commit"),
        );
        default_types.insert(
            CommitType::Test,
            CommitConfig::new("Tests", "create a `test` commit"),
        );

        default_types.insert(
            CommitType::Build,
            CommitConfig::new("Build system", "create a continuous `build` commit"),
        );

        default_types.insert(
            CommitType::Ci,
            CommitConfig::new(
                "Continuous Integration",
                "create a `continuous integration` commit",
            ),
        );

        default_types
    }
}
