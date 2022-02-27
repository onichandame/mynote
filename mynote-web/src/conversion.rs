use async_graphql::MaybeUndefined;

pub trait IntoUniversal<T> {
    fn into_universal(&self) -> T;
}

impl IntoUniversal<Option<Option<String>>> for MaybeUndefined<String> {
    fn into_universal(&self) -> Option<Option<String>> {
        match self {
            MaybeUndefined::Null => Some(None),
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Value(val) => Some(Some(val.to_owned())),
        }
    }
}
