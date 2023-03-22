use std::fmt::Display;
use std::path::PathBuf;

/// The type representing a version segment.
pub type VersionNumber = u16;

/// Parse a version number from a string.
pub fn parse_version_from_str(
    version_str: &str,
) -> Result<Option<(VersionNumber, VersionNumber, VersionNumber)>, String> {
    if version_str.is_empty() {
        return Ok(None);
    }

    let version_split = version_str.split('.').collect::<Vec<_>>();

    match version_split.len() {
        3 => {
            let version_nums = version_split
                .iter()
                .filter_map(|&v| v.parse::<VersionNumber>().ok())
                .collect::<Vec<_>>();

            match version_nums.len() {
                3 => Ok(Some((version_nums[0], version_nums[1], version_nums[2]))),
                _ => Err(format!("failed to parse version: {}", version_str)),
            }
        }
        _ => Err(format!("failed to parse version: {}", version_str)),
    }
}

/// Parse a version number from a file path.
pub fn parse_version_from_path(
    path: PathBuf,
) -> Result<(VersionNumber, VersionNumber, VersionNumber), String> {
    let version_str = path.file_name().unwrap();
    let version_str = PathBuf::from(version_str)
        .with_extension("")
        .to_str()
        .unwrap()
        .to_owned();
    let version_str = version_str.replace("_", ".");
    let (_, version_str) = version_str.split_at(1);
    parse_version_from_str(version_str).map(|v| v.unwrap())
}

/// A representation of an application version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(VersionNumber, VersionNumber, VersionNumber);

impl Version {
    /// Get the identifier of a file corresponding to a version migration.
    pub fn file_id(&self) -> String {
        format!("_{}_{}_{}", self.0, self.1, self.2)
    }
}

impl From<(VersionNumber, VersionNumber, VersionNumber)> for Version {
    fn from(value: (VersionNumber, VersionNumber, VersionNumber)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}.{}.{}", self.0, self.1, self.2))
    }
}
