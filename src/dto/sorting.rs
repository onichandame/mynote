use async_graphql::{Enum, InputObject};
use sorting::{SortDirection, Sorting};

#[derive(InputObject)]
#[graphql(name = "Sorting")]
pub struct SortingDTO {
    pub field: String,
    pub direction: SortDirectionDTO,
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
#[graphql(name = "SortDirection")]
pub enum SortDirectionDTO {
    ASC,
    DESC,
}

impl Into<Sorting> for SortingDTO {
    fn into(self) -> Sorting {
        Sorting {
            field: self.field,
            direction: self.direction.into(),
        }
    }
}

impl Into<SortDirection> for SortDirectionDTO {
    fn into(self) -> SortDirection {
        match self {
            Self::ASC => SortDirection::ASC,
            Self::DESC => SortDirection::DESC,
        }
    }
}
