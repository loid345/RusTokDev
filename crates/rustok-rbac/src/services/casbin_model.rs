const DEFAULT_CASBIN_MODEL: &str = include_str!("../../config/casbin_model.conf");

pub fn default_casbin_model() -> &'static str {
    DEFAULT_CASBIN_MODEL
}

#[cfg(test)]
mod tests {
    use super::default_casbin_model;

    #[test]
    fn model_contains_core_sections() {
        let model = default_casbin_model();
        assert!(model.contains("[request_definition]"));
        assert!(model.contains("[policy_definition]"));
        assert!(model.contains("[role_definition]"));
        assert!(model.contains("[matchers]"));
    }

    #[test]
    fn model_declares_tenant_domain_field() {
        let model = default_casbin_model();
        assert!(model.contains("r = sub, dom, obj, act"));
        assert!(model.contains("p = sub, dom, obj, act"));
        assert!(model.contains("g = _, _, _"));
    }
}
