use std::sync::Arc;

use axum::extract::FromRef;

use crate::{AppConfig, database::DbPool};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: DbPool,
}

impl AppState {
    pub fn new(config: AppConfig, db: DbPool) -> Self {
        Self {
            config: Arc::new(config),
            db,
        }
    }
}

impl FromRef<AppState> for Arc<AppConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}
