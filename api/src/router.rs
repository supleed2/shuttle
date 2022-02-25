use std::collections::HashMap;
use rocket::tokio::sync::RwLock;
use lib::{DeploymentId, Host};

#[derive(Default)]
/// Deployment router which figures out which `DeploymentId`
/// a `Host` corresponds to
pub(crate) struct Router {
    table: RwLock<HashMap<Host, DeploymentId>>
}

impl Router {
    /// Promotes a new `DeploymentId` to a give `Host`. Optionally returns
    /// the old `DeploymentId` if it existed.
    pub(crate) async fn promote(&self, host: Host, id: DeploymentId) -> Option<DeploymentId> {
        self.table.write().await.insert(host, id)
    }

    /// Gets a `DeploymentId` for a given `Host`. Returns `None` if it
    /// does not exist.
    pub(crate) async fn id_for_host(&self, host: &Host) -> Option<DeploymentId> {
        self.table.read().await.get(host).map(|id| id.clone())
    }
}


