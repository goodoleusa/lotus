// This file is part of Lotus Project, a web security scanner written in Rust based on Lua scripts.
// For details, please see https://github.com/rusty-sec/lotus/
//
// Copyright (c) 2022 - Khaled Nassar
//
// Please note that this file was originally released under the GNU General Public License as
// published by the Free Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for the specific language governing permissions
// and limitations under the License.

use crate::utils::bar::GLOBAL_PROGRESS_BAR;
use reqwest::{
    header::HeaderMap,
    redirect, Client, Method, Proxy,
};
use std::collections::HashMap;
use std::time::Duration;

use super::config::{REQUESTS_LIMIT, REQUESTS_SENT, SLEEP_TIME, VERBOSE_MODE};
use super::http_lua_api::{MultiPart, Sender};
use super::response::HttpResponse;
use super::utils::create_form;

impl Sender {
    /// Build your own http request module with user option
    ///
    pub fn init(headers: HeaderMap, proxy: Option<String>, timeout: u64, redirects: u32) -> Sender {
        Sender {
            headers,
            timeout,
            redirects,
            proxy,
            merge_headers: true,
            http_options: super::http_lua_api::HttpVersion::default(),
        }
    }

    fn build_client(
        &self,
        timeout: u64,
        headers: HeaderMap,
        redirects: u32,
        proxy: Option<String>,
    ) -> Result<reqwest::Client, reqwest::Error> {
        let http1_only = self.http_options.http1_only;
        let http2_only = self.http_options.http2_only;

        let mut builder = Client::builder();
        builder = builder.timeout(Duration::from_secs(timeout));
        builder = builder.redirect(redirect::Policy::custom(move |attempt| {
            if attempt.previous().len() != redirects as usize {
                attempt.follow()
            } else {
                attempt.stop()
            }
        }));
        builder = builder.default_headers(headers);

        if let Some(the_proxy) = proxy {
            builder = builder.proxy(Proxy::all(the_proxy).unwrap());
        } else if let Some(the_proxy) = proxy {
            builder = builder.proxy(Proxy::all(the_proxy).unwrap());
        } else {
            builder = builder.no_proxy();
        }

        builder = builder.http1_title_case_headers();
        builder = builder.no_hickory_dns();
        builder = builder.danger_accept_invalid_certs(true);

        if http1_only {
            builder = builder.http1_only();
        } else if http2_only {
            builder = builder.http2_prior_knowledge();
        }

        builder.build()
    }

    /// Send http request to custom url with user options (proxy, headers, etc.)
    /// the response should be HashMap with RespType enum
    ///
    /// for Lua API
    /// ```lua
    /// local resp = http:send("GET","http://google.com")
    /// print(resp.body:GetStrOrNil())
    ///
    /// -- set proxy/timeout    
    /// http:set_proxy("http://proxysite.com:8080")
    /// http:set_timeout(15)
    /// http:set_redirects(2) // set custom redirects limit
    /// ```
    pub async fn send(
        &self,
        method: &str,
        url: String,
        body: Option<String>,
        multipart: Option<HashMap<String, MultiPart>>,
        request_option: Sender,
    ) -> Result<HttpResponse, mlua::Error> {
        {
            let req_limit = *REQUESTS_LIMIT.lock().await;
            let mut req_sent = REQUESTS_SENT.lock().await;
            if *req_sent >= req_limit {
                let sleep_time = *SLEEP_TIME.lock().await;
                GLOBAL_PROGRESS_BAR
                    .lock()
                    .unwrap()
                    .clone()
                    .unwrap()
                    .println(format!(
                        "The rate limit for requests has been reached. Sleeping for {} seconds...",
                        sleep_time
                    ));
                log::debug!(
                    "The rate limit for requests has been reached. Sleeping for {} seconds...",
                    sleep_time
                );
                std::thread::sleep(Duration::from_secs(sleep_time));
                *req_sent = 1;
                {
                    GLOBAL_PROGRESS_BAR
                        .lock()
                        .unwrap()
                        .clone()
                        .unwrap()
                        .println("Continuing...")
                };
                log::debug!("Resetting req_sent value to 1");
            } else {
                *req_sent += 1;
            }
        }

        let client = self
            .build_client(
                request_option.timeout,
                request_option.headers.clone(),
                request_option.redirects,
                request_option.proxy,
            )
            .unwrap();
        let request = client
            .request(Method::from_bytes(method.as_bytes()).unwrap(), url.clone())
            .headers(request_option.headers);

        let request = {
            if body.is_some() {
                request.body(body.unwrap())
            } else {
                request
            }
        };
        let request = {
            if multipart.is_none() {
                request
            } else {
                request.multipart(create_form(multipart.unwrap()))
            }
        };
        let response = match request.send().await {
            Ok(resp) => {
                let verbose_mode = *VERBOSE_MODE.lock().await;
                if verbose_mode {
                    let msg = format!("Sent HTTP request: {}", &url);
                    {
                        GLOBAL_PROGRESS_BAR
                            .lock()
                            .unwrap()
                            .clone()
                            .unwrap()
                            .println(&msg)
                    };
                    log::debug!("{}", msg);
                }

                let resp_headers = resp
                    .headers()
                    .iter()
                    .map(|(name, value)| {
                        (
                            name.to_string(),
                            String::from_utf8_lossy(value.as_bytes()).to_string(),
                        )
                    })
                    .collect::<HashMap<String, String>>();

                let version = resp.version();
                let url = resp.url().to_string();
                let status = resp.status();
                let status_code = status.as_u16() as i32;
                let is_redirect = resp.status().is_redirection();
                let body = resp
                    .bytes()
                    .await
                    .map(|b| String::from_utf8_lossy(&b).to_string());
                match body {
                    Ok(body) => Ok(HttpResponse {
                        reason: status.to_string(),
                        version: format!("{:#?}", version),
                        is_redirect,
                        url,
                        status: status_code,
                        body,
                        headers: resp_headers,
                    }),
                    Err(_) => {
                        let err = mlua::Error::RuntimeError("Timeout Body".to_string());
                        log::error!("Timeout Body");
                        Err(err)
                    }
                }
            }
            Err(err) => {
                let error_code = match () {
                    _ if err.is_timeout() => "timeout_error",
                    _ if err.is_connect() => "connection_error",
                    _ if err.is_redirect() => "too_many_redirects",
                    _ if err.is_body() => "request_body_error",
                    _ if err.is_decode() => "decode_error",
                    _ => "external_error",
                };

                let err = mlua::Error::RuntimeError(error_code.to_string());
                Err(err)
            }
        };

        response
    }
}
