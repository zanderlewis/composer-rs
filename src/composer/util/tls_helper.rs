// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct TlsHelper {
}

impl TlsHelper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn checkCertificateHost(certificate: serde_json::Value, hostname: String, cn: Option<String>) -> bool {
        todo!()
    }

    pub fn getCertificateNames(certificate: serde_json::Value) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn getCertificateFingerprint(certificate: String) -> String {
        todo!()
    }

    pub fn isOpensslParseSafe() -> bool {
        todo!()
    }

    fn certNameMatcher(certName: String) -> Option<Box<dyn Fn()>> {
        todo!()
    }

}

