use rustok_core::Permission;

#[derive(Debug, Clone, Copy)]
pub enum ShadowCheck<'a> {
    Single(&'a Permission),
    Any(&'a [Permission]),
    All(&'a [Permission]),
}

impl ShadowCheck<'_> {
    pub fn as_str(self) -> &'static str {
        match self {
            ShadowCheck::Single(_) => "single",
            ShadowCheck::Any(_) => "any",
            ShadowCheck::All(_) => "all",
        }
    }

    pub fn for_each_permission(self, mut visitor: impl FnMut(Permission)) {
        match self {
            ShadowCheck::Single(permission) => visitor(*permission),
            ShadowCheck::Any(permissions) | ShadowCheck::All(permissions) => {
                for permission in permissions {
                    visitor(*permission);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ShadowCheck;
    use rustok_core::{Action, Permission, Resource};

    fn permission(resource: Resource, action: Action) -> Permission {
        Permission::new(resource, action)
    }

    #[test]
    fn shadow_check_string_representation_is_stable() {
        let required = permission(Resource::Users, Action::Read);
        let required_set = [required];

        assert_eq!(ShadowCheck::Single(&required).as_str(), "single");
        assert_eq!(ShadowCheck::Any(&required_set).as_str(), "any");
        assert_eq!(ShadowCheck::All(&required_set).as_str(), "all");
    }

    #[test]
    fn shadow_check_for_each_permission_visits_all_variants() {
        let single_required = permission(Resource::Users, Action::Read);
        let required_set = [
            permission(Resource::Users, Action::Read),
            permission(Resource::Users, Action::Update),
        ];

        let mut single = Vec::new();
        ShadowCheck::Single(&single_required)
            .for_each_permission(|permission| single.push(permission));

        let mut many = Vec::new();
        ShadowCheck::All(&required_set).for_each_permission(|permission| many.push(permission));

        assert_eq!(single, vec![single_required]);
        assert_eq!(many, required_set.to_vec());
    }
}
