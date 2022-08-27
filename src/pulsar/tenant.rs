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

use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub struct CreateTenantReq {
    host: String,
    port: u16,
    tenant: String,
    allowed_clusters: Vec<String>,
}

pub async fn create_tenant(req: web::Json<CreateTenantReq>) -> HttpResponse {
    match create_tenant_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PutTenantInternalReq {
    allowed_clusters: Vec<String>,
}

async fn create_tenant_internal(req: CreateTenantReq) -> Result<(), Box<dyn std::error::Error>> {
    let put_tenant_req = PutTenantInternalReq {
        allowed_clusters: req.allowed_clusters,
    };
    let response = reqwest::Client::new()
        .put(format!("http://{}:{}/admin/v2/tenants/{}", req.host, req.port, req.tenant))
        .json(&put_tenant_req)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        match response.text().await {
            Ok(text) => {
                log::error!("create tenant failed: {}", text);
                Err(format!("create tenant failed: {}", text).into())
            },
            Err(err) => {
                Err(format!("create tenant failed {}", err).into())
            },
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteTenantReq {
    host: String,
    port: u16,
    tenant: String,
}

pub async fn delete_tenant(req: web::Json<DeleteTenantReq>) -> HttpResponse {
    match delete_tenant_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn delete_tenant_internal(req: DeleteTenantReq) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .delete(format!("http://{}:{}/admin/v2/tenants/{}", req.host, req.port, req.tenant))
        .send()
        .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        match response.text().await {
            Ok(text) => {
                log::error!("delete tenant failed: {}", text);
                Err(format!("delete tenant failed: {}", text).into())
            },
            Err(err) => {
                Err(format!("delete tenant failed {}", err).into())
            },
        }
    }
}
