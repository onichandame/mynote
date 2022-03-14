use async_graphql::MaybeUndefined;

pub trait IntoUniversal<T> {
    fn into_universal(self) -> T;
}

impl<T> IntoUniversal<Option<Option<T>>> for MaybeUndefined<T> {
    fn into_universal(self) -> Option<Option<T>> {
        match self {
            MaybeUndefined::Null => Some(None),
            MaybeUndefined::Undefined => None,
            MaybeUndefined::Value(val) => Some(Some(val)),
        }
    }
}
