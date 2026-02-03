// Namespace: Composer\Util\Http

#[derive(Debug, Clone, Default)]
pub struct ProxyItem {
}

impl ProxyItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toRequestProxy(&self, scheme: String) -> crate::composer::util::http::request_proxy::RequestProxy {
        todo!()
    }

}

