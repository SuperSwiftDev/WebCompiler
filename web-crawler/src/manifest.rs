use std::path::{Path, PathBuf};

// ————————————————————————————————————————————————————————————————————————————
// HELPERS
// ————————————————————————————————————————————————————————————————————————————

// ————————————————————————————————————————————————————————————————————————————
// MANIFEST - DATA MODEL
// ————————————————————————————————————————————————————————————————————————————

pub mod specification {
    use std::path::{Path, PathBuf};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WhitelistSpec {
        pub domains: Vec<String>,
    }

    impl WhitelistSpec {
        pub fn normalize(self, _: &Path) -> Self {
            Self {
                domains: self.domains,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlacklistSpec {
        pub protocols: Vec<String>,
    }

    impl BlacklistSpec {
        pub fn normalize(self, _: &Path) -> Self {
            Self {
                protocols: self.protocols
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ProjectSpec {
        pub id: String,
        pub seed_url: String,
        pub output_dir: PathBuf,
        pub whitelist: WhitelistSpec,
        pub blacklist: BlacklistSpec,
    }

    impl ProjectSpec {
        pub fn normalize(self, base_path: &Path) -> Self {
            Self {
                id: self.id,
                seed_url: self.seed_url,
                output_dir: base_path.join(self.output_dir),
                whitelist: self.whitelist.normalize(base_path),
                blacklist: self.blacklist.normalize(base_path),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ManifestSpec {
        pub projects: Vec<ProjectSpec>,
    }

    impl ManifestSpec {
        pub fn normalize(self, base_path: &Path) -> Self {
            Self {
                projects: self.projects
                    .into_iter()
                    .map(|x| x.normalize(base_path))
                    .collect()
            }
        }
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone)]
pub struct Manifest {
    spec: specification::ManifestSpec,
    #[allow(unused)]
    file_path: PathBuf,
}

impl Manifest {
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let file_path = file_path.as_ref();
        let file = std::fs::read_to_string(file_path)?;
        let spec = toml::from_str::<specification::ManifestSpec>(&file)?;
        let base_path = file_path.parent().unwrap();
        let spec = spec.normalize(&base_path);
        Ok(Self {
            spec,
            file_path: file_path.to_path_buf(),
        })
    }
    pub fn get_project(&self, id: impl AsRef<str>) -> Option<&specification::ProjectSpec> {
        self.spec.projects.iter().find(|project| {
            project.id == id.as_ref()
        })
    }
}


