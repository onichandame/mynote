use sea_orm::{ColumnTrait, Condition, Value};

#[derive(Default, Clone)]
pub struct Filter<TData: Clone + Into<Value>> {
    pub eq: Option<TData>,
    /// is null or is not null
    pub null: Option<bool>,
    pub lt: Option<TData>,
    pub lte: Option<TData>,
    pub gt: Option<TData>,
    pub gte: Option<TData>,
    pub like: Option<String>,
    pub and: Option<Vec<Filter<TData>>>,
    pub or: Option<Vec<Filter<TData>>>,
    pub not: Option<Box<Filter<TData>>>,
}

impl<TData: Clone + Into<Value>> Filter<TData> {
    /// build filter into a SeaORM filter condition
    pub fn build<TCol: ColumnTrait>(&self, col: TCol) -> Condition {
        let mut filter = Condition::all();
        if let Some(eq) = &self.eq {
            filter = filter.add(col.eq(eq.clone()));
        }
        if let Some(null) = &self.null {
            if *null {
                filter = filter.add(col.is_null());
            } else {
                filter = filter.add(col.is_not_null());
            }
        }
        if matches!(self.null, Some(true)) {}
        if let Some(lt) = &self.lt {
            filter = filter.add(col.lt(lt.clone()));
        }
        if let Some(lte) = &self.lte {
            filter = filter.add(col.lte(lte.clone()));
        }
        if let Some(gt) = &self.gt {
            filter = filter.add(col.gt(gt.clone()));
        }
        if let Some(gte) = &self.gte {
            filter = filter.add(col.gte(gte.clone()));
        }
        if let Some(like) = &self.like {
            filter = filter.add(col.like(&like));
        }
        if let Some(and) = &self.and {
            for child_filter in and {
                filter = filter.add(child_filter.build(col.clone()));
            }
        }
        if let Some(or) = &self.or {
            let mut cond = Condition::any();
            for child_filter in or {
                cond = cond.add(child_filter.build(col.clone()));
            }
            filter = filter.add(cond);
        }
        if let Some(not) = &self.not {
            filter = filter.add(Condition::all().not().add(not.as_ref().build(col.clone())));
        }
        filter
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseBackend, EntityTrait, QueryFilter, QueryTrait};

    use super::*;

    #[test]
    fn filter_eq() {
        assert_eq!(
            build_query(
                Filter {
                    eq: Some(""),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.eq(""))),
        );
    }

    #[test]
    fn filter_null() {
        assert_eq!(
            build_query(
                Filter::<String> {
                    null: Some(true),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.is_null())),
        );
        assert_eq!(
            build_query(
                Filter::<String> {
                    null: Some(false),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.is_not_null())),
        )
    }

    #[test]
    fn filter_lt() {
        assert_eq!(
            build_query(
                Filter {
                    lt: Some(""),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.lt(""))),
        )
    }

    #[test]
    fn filter_lte() {
        assert_eq!(
            build_query(
                Filter {
                    lte: Some(""),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.lte(""))),
        )
    }

    #[test]
    fn filter_gt() {
        assert_eq!(
            build_query(
                Filter {
                    gt: Some(""),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.gt(""))),
        )
    }

    #[test]
    fn filter_gte() {
        assert_eq!(
            build_query(
                Filter {
                    gte: Some(""),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.gte(""))),
        )
    }

    #[test]
    fn filter_and() {
        assert_eq!(
            build_query(
                Filter {
                    and: Some(vec!(
                        Filter {
                            lt: Some(""),
                            ..Default::default()
                        },
                        Filter {
                            gt: Some(""),
                            ..Default::default()
                        }
                    )),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(
                Condition::all()
                    .add(Condition::all().add(model::Column::Text.lt("")))
                    .add(Condition::all().add(model::Column::Text.gt("")))
            ),
        )
    }

    #[test]
    fn filter_or() {
        assert_eq!(
            build_query(
                Filter {
                    or: Some(vec!(
                        Filter {
                            lt: Some(""),
                            ..Default::default()
                        },
                        Filter {
                            gt: Some(""),
                            ..Default::default()
                        }
                    )),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(
                Condition::any()
                    .add(Condition::all().add(model::Column::Text.lt("")))
                    .add(Condition::all().add(model::Column::Text.gt("")))
            ),
        )
    }

    #[test]
    fn filter_like() {
        assert_eq!(
            build_query(
                Filter::<String> {
                    like: Some("".to_owned()),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(Condition::all().add(model::Column::Text.like(""))),
        )
    }

    #[test]
    fn filter_not() {
        assert_eq!(
            build_query(
                Filter::<String> {
                    not: Some(Box::new(Filter {
                        eq: Some("".to_owned()),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
                .build(model::Column::Text)
            ),
            build_query(
                Condition::all().add(Condition::all().not().add(model::Column::Text.eq("")))
            ),
        )
    }

    fn build_query(condition: Condition) -> String {
        model::Entity::find()
            .filter(condition)
            .build(DatabaseBackend::Sqlite)
            .to_string()
    }

    pub mod model {
        use sea_orm::entity::prelude::*;

        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "test_model")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: i32,
            pub text: String,
            pub integer: i64,
            pub datetime: chrono::NaiveDateTime,
        }

        #[derive(Copy, Clone, Debug, EnumIter)]
        pub enum Relation {}

        impl RelationTrait for Relation {
            fn def(&self) -> RelationDef {
                panic!("No RelationDef")
            }
        }

        impl ActiveModelBehavior for ActiveModel {}
    }
}
