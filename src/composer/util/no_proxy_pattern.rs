// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct NoProxyPattern {
}

impl NoProxyPattern {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test(&self, url: String) -> bool {
        todo!()
    }

    pub(crate) fn getUrlData(&self, url: String) {
        todo!()
    }

    pub(crate) fn r#match(&self, index: i64, hostName: String, url: serde_json::Value) -> bool {
        todo!()
    }

    pub(crate) fn matchRange(&self, network: serde_json::Value, target: serde_json::Value) -> bool {
        todo!()
    }

    fn getRule(&self, index: i64, hostName: String) -> Option<serde_json::Value> {
        todo!()
    }

    fn ipCheckData(&self, host: String, ipdata: Option<serde_json::Value>, allowPrefix: bool) -> bool {
        todo!()
    }

    fn ipGetAddr(&self, host: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn ipGetMask(&self, prefix: i64, size: i64) -> String {
        todo!()
    }

    fn ipGetNetwork(&self, rangeIp: String, size: i64, prefix: i64) -> Vec<serde_json::Value> {
        todo!()
    }

    fn ipMapTo6(&self, binary: String, size: i64) -> String {
        todo!()
    }

    fn makeData(&self, host: String, port: i64, ipdata: Option<serde_json::Value>) -> serde_json::Value {
        todo!()
    }

    fn makeIpData(&self, ip: String, size: i64, netmask: Option<String>) -> serde_json::Value {
        todo!()
    }

    fn splitHostPort(&self, hostName: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn validateInt(&self, int: String, min: i64, max: i64) -> bool {
        todo!()
    }

}

