use crate::common::db;
use crate::App;
use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse
};
use axum_login::{AuthUser, AuthnBackend, UserId};
use sea_orm::{
    prelude::DateTime,
    ColumnTrait,
    Condition,
    DatabaseConnection,
    EntityTrait,
    FromQueryResult,
    QueryFilter
};
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    sync::Arc
};

type AuthSession = axum_login::AuthSession<Backend>;

pub async fn login(
    State(app): State<Arc<App>>,
    mut auth_session: AuthSession,
    axum::Form(creds): axum::Form<Credentials>,
) -> impl IntoResponse {
    match auth_session.authenticate(Credentials {username: creds.username, password: creds.password}).await {
        Ok(Some(x)) => {
            auth_session.login(&x).await;
            (StatusCode::OK, crate::templates::get_success_json("Authentication successful"))
        }
        Ok(None) => {
            (StatusCode::UNAUTHORIZED, crate::templates::get_error_json("Invalid username or password".to_string()))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, crate::templates::get_error_json(err.to_string()))
        }
    }
}

pub async fn logout(
    State(app): State<Arc<App>>,
    mut auth_session: AuthSession,
) -> impl IntoResponse {
    auth_session.logout().await;
    (StatusCode::OK, crate::templates::get_success_json("Logout successful"))
}

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: DateTime,
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        "awd".as_bytes()
    }
}

#[derive(Clone)]
pub struct Backend {
    db: DatabaseConnection,
}


impl Backend {
    pub(crate) async fn create() -> Self {
        Backend {
            db: db::connect(None).await,
        }
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = sea_orm::error::DbErr;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let password_hash = &*creds.password.clone();

        let user = entity::user::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::user::Column::Username.eq(creds.username))
                    .add(entity::user::Column::Password.eq(password_hash)),
            )
            .into_model::<User>()
            .one(&self.db)
            .await;
        match user {
            Ok(u) => {
                /*if let Some(u) = u {
                    let x = bcrypt::verify(password_hash, &*u.password);
                    match x {
                        Ok(x) => {
                            if x {
                                Ok(Some(u))
                            } else {
                                Ok(None)
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err);
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None)
                }*/
                if let Some(u) = u {
                    if u.password.eq(&creds.password) {
                        Ok(Some(u))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(err),
        }
    }

    fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> impl Future<Output = Result<Option<Self::User>, Self::Error>> + Send {
        entity::user::Entity::find_by_id(*user_id)
            .into_model::<User>()
            .one(&self.db)
    }
}



#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credentials {
    username: String,
    password: String,
}
