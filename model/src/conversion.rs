use sea_orm::{ActiveValue, NotSet, Set};

pub trait IntoActiveValue<T> {
    fn into_active_value(&self) -> T;
}

impl IntoActiveValue<ActiveValue<String>> for String {
    fn into_active_value(&self) -> ActiveValue<String> {
        Set(self.to_owned())
    }
}

impl IntoActiveValue<ActiveValue<String>> for Option<String> {
    fn into_active_value(&self) -> ActiveValue<String> {
        match self {
            None => NotSet,
            Some(val) => Set(val.to_owned()),
        }
    }
}

impl IntoActiveValue<ActiveValue<Option<String>>> for Option<String> {
    fn into_active_value(&self) -> ActiveValue<Option<String>> {
        Set(self.to_owned())
    }
}

impl IntoActiveValue<ActiveValue<Option<String>>> for Option<Option<String>> {
    fn into_active_value(&self) -> ActiveValue<Option<String>> {
        match self {
            None => NotSet,
            Some(val) => Set(val.to_owned()),
        }
    }
}

impl IntoActiveValue<ActiveValue<i32>> for i32 {
    fn into_active_value(&self) -> ActiveValue<i32> {
        Set(self.to_owned())
    }
}

impl IntoActiveValue<ActiveValue<i32>> for Option<i32> {
    fn into_active_value(&self) -> ActiveValue<i32> {
        match self {
            None => NotSet,
            Some(val) => Set(val.to_owned()),
        }
    }
}
