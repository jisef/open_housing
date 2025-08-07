use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"

create table if not exists room
(
    room_pk      serial
        primary key,
    number       integer default 0,
    room_name    varchar,
    capacity     integer,
    max_capacity integer,
    is_apartment boolean,
    has_kitchen  boolean,
    bedrooms     integer
);

alter table room
    owner to root;

create table if not exists booking
(
    booking_pk           serial
        primary key,
    date_start           date not null,
    date_end             date not null,
    with_breakfast       boolean   default true,
    num_full_aged_guests integer,
    num_children         integer,
    checked_in           boolean   default false,
    created_at           timestamp default CURRENT_TIMESTAMP,
    checked_out          boolean   default false
);

alter table booking
    owner to root;

create table if not exists room_booking
(
    booking_fk integer not null
        references booking,
    room_fk    integer not null
        references room,
    num_people integer,
    primary key (booking_fk, room_fk)
);

alter table room_booking
    owner to root;

create table if not exists roomphotos
(
    roomphoto_pk serial
        primary key,
    room_fk      integer not null
        references room,
    photo        bytea   not null
);

alter table roomphotos
    owner to root;
        "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"
        DROP TABLE IF EXISTS roomphotos;
        DROP TABLE IF EXISTS room_booking;
        DROP TABLE IF EXISTS room;
        DROP TABLE IF EXISTS booking;
        "#).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
