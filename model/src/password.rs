//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "passwords")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub uuid: String,
    pub created_at: DateTimeUtc,
    pub updated_at: Option<DateTimeUtc>,
    pub deleted_at: Option<DateTimeUtc>,
    pub is_local: bool,
    pub user_id: i32,
    pub group_id: Option<i32>,
    pub title: String,
    pub password: String,
    pub url: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::password_group::Entity",
        from = "Column::GroupId",
        to = "super::password_group::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PasswordGroup,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::password_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PasswordGroup.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
