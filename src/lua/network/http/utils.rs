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

use mlua::Value;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    multipart::{Form, Part},
};
use std::collections::HashMap;

use super::http_lua_api::MultiPart;

/// Convert a serde_json::Value to a Lua value
pub fn json_to_lua<'lua>(
    lua: &'lua mlua::Lua,
    value: &serde_json::Value,
) -> Result<Value<'lua>, mlua::Error> {
    match value {
        serde_json::Value::Null => Ok(Value::Nil),
        serde_json::Value::Bool(b) => Ok(Value::Boolean(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(Value::Number(f))
            } else {
                Err(mlua::Error::RuntimeError("Invalid JSON number".to_string()))
            }
        }
        serde_json::Value::String(s) => Ok(Value::String(lua.create_string(s)?)),
        serde_json::Value::Array(arr) => {
            let table = lua.create_table()?;
            for (i, val) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, val)?)?;
            }
            Ok(Value::Table(table))
        }
        serde_json::Value::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj.iter() {
                table.set(key.as_str(), json_to_lua(lua, val)?)?;
            }
            Ok(Value::Table(table))
        }
    }
}

/// Create a multipart form from a HashMap of MultiPart values
pub fn create_form(multipart: HashMap<String, MultiPart>) -> Form {
    let mut form = Form::new();
    for (key, part) in multipart {
        let mut builder = Part::text(part.content);
        if let Some(filename) = part.filename {
            builder = builder.file_name(filename);
        }
        if let Some(content_type) = part.content_type {
            builder = builder.mime_str(&content_type).unwrap();
        }
        if let Some(headers) = part.headers {
            let mut current_headers = HeaderMap::new();
            headers.iter().for_each(|(name, value)| {
                current_headers.insert(
                    HeaderName::from_bytes(name.as_bytes()).unwrap(),
                    HeaderValue::from_bytes(value.as_bytes()).unwrap(),
                );
            });
            builder = builder.headers(current_headers);
        }
        form = form.part(key, builder);
    }
    form
}
