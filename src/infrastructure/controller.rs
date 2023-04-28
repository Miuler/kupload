use crate::domain::entity::upload_info::UploadInfo;
use axum::response::Html;
use axum::Json;
// use k8s_openapi::api::core::v1::{Namespace, Pod};
// use kube::{Api, Client, ResourceExt};
// use crate::domain::entity::error::{KUploadError, Result};

#[derive(Clone, Debug, Default)]
pub struct Controller {}

impl Controller {
    pub async fn test() -> Html<&'static str> {
        Html("<html><body>Hola</body></html>")
    }

    #[axum::debug_handler]
    pub async fn hello_world() -> Json<UploadInfo> {
        Json(UploadInfo {
            id: "id".to_string(),
            path: "path".to_string(),
        })
    }

    // async fn list_pods(&self) -> Result<()> {
    //     let client = Client::try_default()
    //         .await
    //         .map_err(|_e| KUploadError::KubeError(_e.to_string()))?;
    //
    //     let namespaces = Api::<Namespace>::all(client.clone());
    //     namespaces.list(&Default::default())
    //         .await
    //         .map_err(|_e| KUploadError::KubeError(_e.to_string()))?
    //         .items
    //         .iter()
    //         .for_each(|ns| {
    //             let name = ns.metadata.name.clone().unwrap();
    //             println!("namespace: {}", name)
    //         });
    //
    //
    //     // let pods = Api::<Pod>::all(client);
    //     let pods = Api::<Pod>::namespaced(client.clone(), "genesis");
    //     pods.list(&Default::default())
    //         .await
    //         .map_err(|_e| KUploadError::KubeError(_e.to_string()))?
    //         .items
    //         .iter()
    //         .for_each(|pod| {
    //             let (name, namespace) = (
    //                 pod.metadata.name.clone().unwrap(),
    //                 pod.namespace().unwrap()
    //             );
    //             println!("namespace: {}  pod: {}", namespace, name)
    //         });
    //
    //     Ok(())
    // }
}
