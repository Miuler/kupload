use axum::Router;
use axum::routing::get;

use crate::domain::entity::error::{KUploadError, Result};
use crate::infrastructure::controller::Controller;

#[derive(Debug)]
pub struct Server {
    address: String,
    controller: Controller,
}

impl Server {
    pub fn builder(&self) -> ServerBuilder {
        ServerBuilder {
            address: Some(self.address.clone()),
            ..ServerBuilder::default()
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("run server: {:?}", self);
        let app = Router::new()
            .route("/", get(Controller::hello_world))
            .route("/info", get(Controller::test))
            .with_state(self.controller.clone());

        let server =
            axum::Server::bind(&self.address.parse().unwrap()).serve(app.into_make_service());

        println!("server.await iniciando");
        server
            .await
            .map_err(|e| KUploadError::KubeError(e.to_string()))?;
        println!("server.await termino");

        Ok(())
    }
}

/// This builder is used to build a server.
#[derive(Debug, Default)]
pub struct ServerBuilder {
    address: Option<String>,
    controller: Option<Controller>,
}

impl ServerBuilder {
    const DEFAULT_ADDRESS: &'static str = "0.0.0.0:8080";

    pub fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }

    /// This function is used to build a server with default values.
    pub async fn build_with_default() -> Result<Server> {
        ServerBuilder::default().build().await
    }

    /// This function is used to build a server.
    pub async fn build(&mut self) -> Result<Server> {
        // let listener: tokio::net::TcpListener = if self.listener.is_some() {
        //     self.listener.take().unwrap()
        // } else {
        //     let address = self.address
        //         .get_or_insert("0.0.0.0:6000".to_string())
        //         .clone();
        //     tokio::net::TcpListener::bind(address)
        //         .await
        //         .map_err(|e| KUploadError::KubeError(e.to_string()))?
        // };

        Ok(Server {
            address: self
                .address
                .get_or_insert(Self::DEFAULT_ADDRESS.to_string())
                .clone(),
            controller: self.controller.get_or_insert(Controller::default()).clone(),
        })
    }
}
