// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::time::Duration;
use actix_web::{HttpResponse, web};
use buruma::{ACL, CreateMode, ZooKeeper};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNodeReq {
    host: String,
    port: u16,
    node_path: String,
    data: String,
}

pub async fn create_node(req: web::Json<CreateNodeReq>) -> HttpResponse {
    match create_node_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn create_node_internal(req: CreateNodeReq) -> Result<(), Box<dyn std::error::Error>> {
    let mut zk = ZooKeeper::new(format!("{}:{}", req.host, req.port).as_str(),
                                Duration::from_secs(5))
        .await
        .unwrap();
    let path = zk.create(req.node_path.as_str(), Some(req.data.as_bytes()),
                         ACL::world_acl(), CreateMode::Persistent)
        .await
        .unwrap();
    log::info!("create zookeeper path {:?}", path);
    Ok(())
}

#[derive(Deserialize)]
pub struct DeleteNodeReq {
    host: String,
    port: u16,
    node_path: String,
}

pub async fn delete_node(req: web::Json<DeleteNodeReq>) -> HttpResponse {
    match delete_node_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn delete_node_internal(req: DeleteNodeReq) -> Result<(), Box<dyn std::error::Error>> {
    let mut zk = ZooKeeper::new(format!("{}:{}", req.host, req.port).as_str(),
                                Duration::from_secs(5))
        .await
        .unwrap();
    zk.delete(req.node_path.as_str());
    Ok(())
}
