//! Auto-generated from composer.json

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJson {
    pub authors: Option<Vec<ComposerJsonAuthors>>,
    pub autoload: Option<ComposerJsonAutoload>,
    #[serde(rename = "autoload-dev")]
    pub autoload_dev: Option<ComposerJsonAutoloadDev>,
    pub bin: Option<Vec<String>>,
    pub config: Option<ComposerJsonConfig>,
    pub description: Option<String>,
    pub extra: Option<ComposerJsonExtra>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub name: Option<String>,
    pub require: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "require-dev")]
    pub require_dev: Option<std::collections::HashMap<String, String>>,
    pub scripts: Option<ComposerJsonScripts>,
    #[serde(rename = "scripts-descriptions")]
    pub scripts_descriptions: Option<ComposerJsonScriptsDescriptions>,
    pub suggest: Option<ComposerJsonSuggest>,
    pub support: Option<ComposerJsonSupport>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonSupport {
    pub irc: Option<String>,
    pub issues: Option<String>,
    pub security: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonSuggest {
    #[serde(rename = "ext-curl")]
    pub ext_curl: Option<String>,
    #[serde(rename = "ext-openssl")]
    pub ext_openssl: Option<String>,
    #[serde(rename = "ext-zip")]
    pub ext_zip: Option<String>,
    #[serde(rename = "ext-zlib")]
    pub ext_zlib: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonScriptsDescriptions {
    pub compile: Option<String>,
    pub phpstan: Option<String>,
    pub test: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonScripts {
    pub compile: Option<String>,
    pub phpstan: Option<String>,
    pub test: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonExtra {
    #[serde(rename = "branch-alias")]
    pub branch_alias: Option<ComposerJsonExtraBranchAlias>,
    pub phpstan: Option<ComposerJsonExtraPhpstan>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonExtraPhpstan {
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonExtraBranchAlias {
    #[serde(rename = "dev-main")]
    pub dev_main: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonConfig {
    pub platform: Option<ComposerJsonConfigPlatform>,
    #[serde(rename = "platform-check")]
    pub platform_check: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonConfigPlatform {
    pub php: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonAutoloadDev {
    #[serde(rename = "exclude-from-classmap")]
    pub exclude_from_classmap: Option<Vec<String>>,
    #[serde(rename = "psr-4")]
    pub psr_4: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonAutoload {
    #[serde(rename = "psr-4")]
    pub psr_4: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComposerJsonAuthors {
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub name: Option<String>,
}

