use derive_builder::{Builder, UninitializedFieldError};
use std::fmt;
use strum;
use thiserror::Error;

#[derive(Clone, Debug, strum::Display)]
enum Type {
    #[strum(to_string = "fix")]
    Fix,
    #[strum(to_string = "feat")]
    Feat,
    #[strum(to_string = "{0}")]
    Custom(String),
}

#[derive(Clone, Debug)]
struct Scope(String);

#[derive(Builder, Debug)]
#[builder(build_fn(error = "HeaderBuilderError"))]
struct Header {
    ctype: Type,
    #[builder(default = None)]
    scope: Option<Scope>,
    description: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum HeaderBuilderError {
    #[error("missing required field: {0}")]
    MissingRequiredFieldError(String),
}

impl From<UninitializedFieldError> for HeaderBuilderError {
    fn from(value: UninitializedFieldError) -> Self {
        Self::MissingRequiredFieldError(value.field_name().to_string())
    }
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
        let header = HeaderBuilder::default()
            .ctype(ctype)
            .scope(scope)
            .description(desc.into())
            .build()
            .expect("unexpected error building header");

        assert_eq!(format!("{}", header), expect)
    }

    #[rstest]
    #[case::missing_ctype(None, Some("test description"), HeaderBuilderError::MissingRequiredFieldError("ctype".into()))]
    #[case::missing_description(Some(Type::Feat), None, HeaderBuilderError::MissingRequiredFieldError("description".into()))]
    fn should_fail_to_build_header(
        #[case] ctype: Option<Type>,
        #[case] desc: Option<&str>,
        #[case] expect: HeaderBuilderError,
    ) {
        let mut header = HeaderBuilder::default();

        if let Some(t) = ctype {
            header.ctype(t);
        }

        if let Some(d) = desc {
            header.description(d.into());
        }

        let err = header
            .build()
            .expect_err("should have failed to build header");

        assert_eq!(err, expect)
    }
}
