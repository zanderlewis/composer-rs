// Namespace: Composer\Filter\PlatformRequirementFilter

#[derive(Debug, Clone, Default)]
pub struct PlatformRequirementFilterFactory {
}

impl PlatformRequirementFilterFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fromBoolOrList(boolOrList: serde_json::Value) -> Box<dyn crate::composer::filter::platform_requirement_filter::platform_requirement_filter_interface::PlatformRequirementFilterInterface> {
        todo!()
    }

    pub fn ignoreAll() -> Box<dyn crate::composer::filter::platform_requirement_filter::platform_requirement_filter_interface::PlatformRequirementFilterInterface> {
        todo!()
    }

    pub fn ignoreNothing() -> Box<dyn crate::composer::filter::platform_requirement_filter::platform_requirement_filter_interface::PlatformRequirementFilterInterface> {
        todo!()
    }

}

