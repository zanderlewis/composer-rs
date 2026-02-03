//! Auto-generated from composer.lock

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLock {
    pub _readme: Option<Vec<String>>,
    pub aliases: Option<Vec<serde_json::Value>>,
    #[serde(rename = "content-hash")]
    pub content_hash: Option<String>,
    #[serde(rename = "minimum-stability")]
    pub minimum_stability: Option<String>,
    pub packages: Option<Vec<ComposerLockPackages>>,
    #[serde(rename = "packages-dev")]
    pub packages_dev: Option<Vec<ComposerLockPackagesDev>>,
    pub platform: Option<ComposerLockPlatform>,
    #[serde(rename = "platform-dev")]
    pub platform_dev: Option<ComposerLockPlatformDev>,
    #[serde(rename = "platform-overrides")]
    pub platform_overrides: Option<ComposerLockPlatformOverrides>,
    #[serde(rename = "plugin-api-version")]
    pub plugin_api_version: Option<String>,
    #[serde(rename = "prefer-lowest")]
    pub prefer_lowest: Option<bool>,
    #[serde(rename = "prefer-stable")]
    pub prefer_stable: Option<bool>,
    #[serde(rename = "stability-flags")]
    pub stability_flags: Option<ComposerLockStabilityFlags>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockStabilityFlags {
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPlatformOverrides {
    pub php: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPlatformDev {
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPlatform {
    #[serde(rename = "ext-json")]
    pub ext_json: Option<String>,
    pub php: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDev {
    pub authors: Option<Vec<ComposerLockPackagesDevAuthors>>,
    pub autoload: Option<ComposerLockPackagesDevAutoload>,
    pub bin: Option<Vec<String>>,
    pub conflict: Option<std::collections::HashMap<String, String>>,
    pub description: Option<String>,
    pub dist: Option<ComposerLockPackagesDevDist>,
    pub extra: Option<ComposerLockPackagesDevExtra>,
    pub funding: Option<Vec<ComposerLockPackagesDevFunding>>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<Vec<String>>,
    pub name: Option<String>,
    #[serde(rename = "notification-url")]
    pub notification_url: Option<String>,
    pub require: Option<ComposerLockPackagesDevRequire>,
    #[serde(rename = "require-dev")]
    pub require_dev: Option<std::collections::HashMap<String, String>>,
    pub source: Option<ComposerLockPackagesDevSource>,
    pub support: Option<ComposerLockPackagesDevSupport>,
    pub time: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevSupport {
    pub docs: Option<String>,
    pub forum: Option<String>,
    pub issues: Option<String>,
    pub security: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevSource {
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevRequire {
    pub php: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevFunding {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevExtra {
    pub phpstan: Option<ComposerLockPackagesDevExtraPhpstan>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevExtraPhpstan {
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevDist {
    pub reference: Option<String>,
    pub shasum: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevAutoload {
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDevAuthors {
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackages {
    pub authors: Option<Vec<ComposerLockPackagesAuthors>>,
    pub autoload: Option<ComposerLockPackagesAutoload>,
    pub bin: Option<Vec<String>>,
    pub conflict: Option<std::collections::HashMap<String, String>>,
    pub description: Option<String>,
    pub dist: Option<ComposerLockPackagesDist>,
    pub extra: Option<ComposerLockPackagesExtra>,
    pub funding: Option<Vec<ComposerLockPackagesFunding>>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<Vec<String>>,
    pub name: Option<String>,
    #[serde(rename = "notification-url")]
    pub notification_url: Option<String>,
    pub provide: Option<std::collections::HashMap<String, String>>,
    pub require: Option<ComposerLockPackagesRequire>,
    #[serde(rename = "require-dev")]
    pub require_dev: Option<std::collections::HashMap<String, String>>,
    pub source: Option<ComposerLockPackagesSource>,
    pub suggest: Option<std::collections::HashMap<String, String>>,
    pub support: Option<ComposerLockPackagesSupport>,
    pub time: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesSupport {
    pub irc: Option<String>,
    pub issues: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesSource {
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesRequire {
    #[serde(rename = "ext-openssl")]
    pub ext_openssl: Option<String>,
    #[serde(rename = "ext-pcre")]
    pub ext_pcre: Option<String>,
    pub php: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesFunding {
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesExtra {
    #[serde(rename = "branch-alias")]
    pub branch_alias: Option<ComposerLockPackagesExtraBranchAlias>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesExtraBranchAlias {
    #[serde(rename = "dev-main")]
    pub dev_main: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesDist {
    pub reference: Option<String>,
    pub shasum: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesAutoload {
    #[serde(rename = "psr-4")]
    pub psr_4: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerLockPackagesAuthors {
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
}

