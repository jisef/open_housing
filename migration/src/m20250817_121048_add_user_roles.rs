use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        conn.execute_unprepared(
            r#"
INSERT INTO "user" (username, password) VALUES ('admin', '$2b$12$Mr73/7lxGq1lPCwXDgdmuO59zNzCdZi94Pi9s5Mm3e2fYZCfPuyLq') ON CONFLICT DO NOTHING; --password: cisco
ALTER TABLE "user" DROP COLUMN "is_admin";

create table permission
(
    permission_id   SERIAL primary key,
    name text not null unique
);

create table user_permission
(
    user_id      integer references "user" (user_id),
    permission_id integer references permission (permission_id),
    primary key (user_id, permission_id)
);

INSERT INTO permission (name)
VALUES ('admin');
INSERT INTO user_permission (user_id, permission_id)
VALUES ((SELECT user_id from "user" WHERE username = 'admin'),
        (SELECT permission_id from permission WHERE name = 'admin'));


            "#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        conn.execute_unprepared(
            r#"
DELETE FROM "user" WHERE username = 'admin';

ALTER TABLE "user" ADD COLUMN "is_admin" boolean;
DROP TABLE user_permission;
DROP TABLE permission;
            "#,
        )
        .await?;
        Ok(())
    }
}
