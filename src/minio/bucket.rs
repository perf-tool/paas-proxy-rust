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
use s3::{Bucket, BucketConfiguration};
use s3::creds::Credentials;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateBucketReq {
    host: String,
    port: u16,
    access_key: String,
    secret_key: String,
    bucket_name: String,
}

pub async fn create_bucket(req: web::Json<CreateBucketReq>) -> HttpResponse {
    match create_bucket_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn create_bucket_internal(req: CreateBucketReq) -> Result<(), Box<dyn std::error::Error>> {
    let region = s3::Region::Custom {
        region: "us-east-1".to_owned(),
        endpoint: format!("http://{}:{}", req.host, req.port),
    };
    let credentials = Credentials::new(Some(req.access_key.as_str()), Some(req.secret_key.as_str()), None, None, None)?;
    let config = BucketConfiguration::default();
    let create_bucket_response = Bucket::create_with_path_style(req.bucket_name.as_str(), region, credentials, config).await?;
    log::info!("create bucket success, resp is {:?}", create_bucket_response.response_text);
    Ok(())
}


#[derive(Deserialize)]
pub struct DeleteBucketReq {
    host: String,
    port: u16,
    access_key: String,
    secret_key: String,
    bucket_name: String,
}

pub async fn delete_bucket(req: web::Json<DeleteBucketReq>) -> HttpResponse {
    match delete_bucket_internal(req.0).await {
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

async fn delete_bucket_internal(req: DeleteBucketReq) -> Result<(), Box<dyn std::error::Error>> {
    let region = s3::Region::Custom {
        region: "us-east-1".to_owned(),
        endpoint: format!("http://{}:{}", req.host, req.port),
    };
    let credentials = Credentials::new(Some(req.access_key.as_str()), Some(req.secret_key.as_str()), None, None, None)?;
    Bucket::new(req.bucket_name.as_str(), region, credentials)?.with_path_style().delete().await?;
    Ok(())
}
