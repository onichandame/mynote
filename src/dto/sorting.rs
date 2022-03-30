use async_graphql::Enum;
use sorting::SortDirection;

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
#[graphql(name = "SortDirection")]
pub enum SortDirectionDTO {
    ASC,
    DESC,
}

impl Into<SortDirection> for SortDirectionDTO {
    fn into(self) -> SortDirection {
        match self {
            Self::ASC => SortDirection::ASC,
            Self::DESC => SortDirection::DESC,
        }
    }
}
