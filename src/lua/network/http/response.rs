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

use mlua::UserData;
use std::collections::HashMap;
use tealr::TypeName;

#[derive(Debug, Clone, TypeName)]
pub struct HttpResponse {
    pub reason: String,
    pub version: String,
    pub is_redirect: bool,
    pub url: String,
    pub status: i32,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl UserData for HttpResponse {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("reason", |_, this| Ok(this.reason.clone()));
        fields.add_field_method_get("version", |_, this| Ok(this.version.clone()));
        fields.add_field_method_get("is_redirect", |_, this| Ok(this.is_redirect));
        fields.add_field_method_get("url", |_, this| Ok(this.url.clone()));
        fields.add_field_method_get("status", |_, this| Ok(this.status));
        fields.add_field_method_get("body", |_, this| Ok(this.body.clone()));
        fields.add_field_method_get("headers", |_, this| Ok(this.headers.clone()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // json() - parse body as JSON and return Lua table
        methods.add_method("json", |lua, this, ()| {
            match serde_json::from_str::<serde_json::Value>(&this.body) {
                Ok(json_value) => super::utils::json_to_lua(lua, &json_value),
                Err(e) => Err(mlua::Error::RuntimeError(format!(
                    "JSON parse error: {}",
                    e
                ))),
            }
        });

        // status_ok() - check if status code is 2xx
        methods.add_method("status_ok", |_, this, ()| {
            Ok(this.status >= 200 && this.status < 300)
        });

        // has_header(name) - case-insensitive header check
        methods.add_method("has_header", |_, this, name: String| {
            let name_lower = name.to_lowercase();
            Ok(this
                .headers
                .keys()
                .any(|k| k.to_lowercase() == name_lower))
        });

        // get_header(name) - get header value (case-insensitive)
        methods.add_method("get_header", |_, this, name: String| {
            let name_lower = name.to_lowercase();
            for (k, v) in &this.headers {
                if k.to_lowercase() == name_lower {
                    return Ok(Some(v.clone()));
                }
            }
            Ok(None)
        });
    }
}
