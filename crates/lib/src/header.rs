use std::fmt;
use strum;

#[derive(Debug, strum::Display)]
enum Type {
    #[strum(to_string = "fix")]
    Fix,
    #[strum(to_string = "feat")]
    Feat,
    #[strum(to_string = "{0}")]
    Custom(String),
}

struct Scope(String);

struct Header {
    ctype: Type,
    scope: Option<Scope>,
    description: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(scope) = &self.scope {
            write!(f, "{}({}): {}", self.ctype, scope.0, self.description)
        } else {
            write!(f, "{}: {}", self.ctype, self.description)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case::fix_no_scope(Type::Fix, None, "test description", "fix: test description")]
    #[case::feat_no_scope(Type::Feat, None, "test description", "feat: test description")]
    #[case::other_no_scope(Type::Custom("build".to_string()), None, "test description", "build: test description")]
    #[case::fix_with_scope(Type::Fix, Some(Scope("mercury".to_string())), "test description", "fix(mercury): test description")]
    #[case::feat_with_scope(Type::Feat, Some(Scope("saturn".to_string())), "test description", "feat(saturn): test description")]
    #[case::other_with_scope(Type::Custom("docs".to_string()), Some(Scope("jupiter".to_string())), "test description", "docs(jupiter): test description")]
    fn should_display_header(
        #[case] ctype: Type,
        #[case] scope: Option<Scope>,
        #[case] desc: &str,
        #[case] expect: &str,
    ) {
        let header = Header {
            ctype,
            scope,
            description: desc.to_string(),
        };

        assert_eq!(format!("{}", header), expect)
    }
}
