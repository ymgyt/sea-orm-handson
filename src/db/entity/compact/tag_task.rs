//! SeaORM Entity. Generated by sea-orm-codegen 0.4.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tag_task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_uuid: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub task_uuid: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagUuid",
        to = "super::tags::Column::Uuid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tags,
    #[sea_orm(
        belongs_to = "super::tasks::Entity",
        from = "Column::TaskUuid",
        to = "super::tasks::Column::Uuid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Tasks,
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tags.def()
    }
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}