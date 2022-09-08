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
use redis::{Commands, IntoConnectionInfo};
use serde::Deserialize;
use crate::redis::cluster_enable;

#[derive(Deserialize)]
pub struct SetKeyReq {
    url: String,
    key: String,
    value: String,
}

pub async fn set_key(req: web::Json<SetKeyReq>, params: web::Query<cluster_enable::Params>) -> HttpResponse {
    match set_key_internal(req.0, params.0.cluster).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn set_key_internal(req: SetKeyReq, cluster: bool) -> Result<(), Box<dyn std::error::Error>> {
    if cluster {
        let redis = redis::cluster::ClusterClient::open(req.url.split(",")
            .map(|s| s.to_string().into_connection_info().unwrap()).collect())?;
        let mut con = redis.get_connection()?;
        con.set(req.key, req.value)?;
    } else {
        let redis = redis::Client::open(req.url.into_connection_info().unwrap())?;
        let mut con = redis.get_connection()?;
        con.set(req.key, req.value)?;
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct DeleteKeyReq {
    url: String,
    key: String,
}

pub async fn delete_key(req: web::Json<DeleteKeyReq>, params: web::Query<cluster_enable::Params>) -> HttpResponse {
    match delete_key_internal(req.0, params.0.cluster).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn delete_key_internal(req: DeleteKeyReq, cluster: bool) -> Result<(), Box<dyn std::error::Error>> {
    if cluster {
        let redis = redis::cluster::ClusterClient::open(req.url.split(",")
            .map(|s| s.to_string().into_connection_info().unwrap()).collect())?;
        let mut con = redis.get_connection()?;
        con.del(req.key)?;
    } else {
        let redis = redis::Client::open(req.url.into_connection_info().unwrap())?;
        let mut con = redis.get_connection()?;
        con.del(req.key)?;
    }
    Ok(())
}
