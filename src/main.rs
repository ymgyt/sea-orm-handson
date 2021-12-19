#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .init();
    db_handson().await
}

async fn db_handson() -> Result<(), anyhow::Error> {
    use sea_orm::{Database, ConnectOptions};

    let mut opt = ConnectOptions::new(env!("DATABASE_URL").to_owned());

    opt.sqlx_logging(true);

    let db = Database::connect(opt).await?;

    use sea_orm::prelude::*;
    use sea_orm::entity::Set;
    use chrono::{DateTime};
    use seaormho::db::entity;

    // 状態をリセットする。
    entity::tasks::Entity::delete_many().exec(&db).await?;
    entity::projects::Entity::delete_many().exec(&db).await?;
    // DELETE FROM "tasks"
    // DELETE FROM "projects"

    // projectのINSERT。
    let project_1 = entity::projects::ActiveModel {
        uuid: Set(String::from("project_1")),
        name: Set(String::from("project_1")),
    };
    project_1.insert(&db).await?;
    // INSERT INTO "projects" ("uuid", "name") VALUES ('project_1', 'project_1') RETURNING "uuid", "name"

    // projectの複数INSERT。
    entity::projects::Entity::insert_many(
        vec![
            entity::projects::ActiveModel {
                uuid: Set(String::from("project_2")),
                name: Set(String::from("project_2")),
            },
            entity::projects::ActiveModel {
                uuid: Set(String::from("project_3")),
                name: Set(String::from("project_3")),
            },
        ]
    ).exec(&db).await?;
    // INSERT INTO "projects" ("uuid", "name")
    // VALUES ('project_2', 'project_2'), ('project_3', 'project_3') RETURNING "uuid"

    // taskのINSERT。
    // NOT NULL fieldをDefault(Unset)にするとSQLエラーになる。
    let task_1 = entity::tasks::ActiveModel {
        uuid: Set(String::from("task_1")),
        project_uuid: Set(String::from("project_1")),
        title: Set(String::from("task_1_title")),
        category: Set(String::from("task_1_category")),
        content: Set(String::from("task_1_content")),
        created_at: Set(DateTime::parse_from_rfc3339("2021-12-19T21:30:00+09:00").unwrap()),
        ..Default::default()
    };
    entity::tasks::Entity::insert(task_1).exec(&db).await?;
    // INSERT INTO "tasks" ("uuid", "project_uuid", "title", "category", "content", "created_at") VALUES ('task_1', 'project_1', 'task_1_title', 'task_1_category', 'task_1_content', '2021-12-19 21:30:00 +09:00') RETURNING "uuid"

    // taskのprimary keyによる取得。
    let task_1 = entity::tasks::Entity::find_by_id("task_1".into()).one(&db).await?.unwrap();
    // SELECT "tasks"."uuid", "tasks"."project_uuid", "tasks"."title", "tasks"."category", "tasks"."content", "tasks"."created_at", "tasks"."deladline" FROM "tasks" WHERE "tasks"."uuid" = 'task_1' LIMIT 1
    assert_eq!(task_1.title.as_str(), "task_1_title");

    // taskのUPDATE。
    let mut task_1: entity::tasks::ActiveModel = task_1.into();
    task_1.content = Set(String::from("task_1_new_content"));
    task_1.update(&db).await?;
    // UPDATE "tasks" SET "content" = 'task_1_new_content' WHERE "tasks"."uuid" = 'task_1' RETURNING "uuid", "project_uuid", "title", "category", "content", "created_at", "deladline"

    Ok(())
}
