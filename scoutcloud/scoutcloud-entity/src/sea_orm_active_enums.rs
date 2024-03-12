//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "deployment_status_type"
)]
pub enum DeploymentStatusType {
    #[sea_orm(string_value = "created")]
    Created,
    #[sea_orm(string_value = "failed")]
    Failed,
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "running")]
    Running,
    #[sea_orm(string_value = "stopped")]
    Stopped,
    #[sea_orm(string_value = "stopping")]
    Stopping,
}
