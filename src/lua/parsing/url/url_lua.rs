use crate::lua::parsing::url::HttpMessage;
use mlua::{ExternalError, ExternalResult, UserData};
use url::Url;

impl UserData for HttpMessage {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("clone", |_, this, ()| {
            let msg = this.clone();
            Ok(msg)
        });
        methods.add_method_mut("new", |_, this, url: String| match Url::parse(&url) {
            Ok(parsed_url) => {
                this.url = Some(parsed_url.clone());
                Ok(parsed_url.to_string())
            }
            Err(err) => Err(err.to_lua_err()),
        });
        methods.add_method(
            "param_set",
            |_, this, (param, payload, remove_content): (String, String, bool)| {
                Ok(this.set_urlvalue(&param, &payload, remove_content))
            },
        );
        methods.add_method(
            "param_set_all",
            |_, this, (payload, remove_content): (String, bool)| {
                Ok(this.change_urlquery(&payload, remove_content))
            },
        );
        methods.add_method("url", |_, this, ()| match &this.url {
            Some(url) => Ok(url.as_str().to_string()),
            None => Err("No url found").to_lua_err(),
        });
        methods.add_method("path", |_, this, ()| match &this.url {
            Some(url) => Ok(url.path().to_string()),
            None => Err("No url found").to_lua_err(),
        });
        methods.add_method("param_str", |_, this, ()| match &this.url {
            Some(url) => Ok(url.query().unwrap_or("").to_string()),
            None => Err("No url found").to_lua_err(),
        });
        methods.add_method("param_list", |_, this, ()| {
            let mut all_params = Vec::new();
            match &this.url {
                Some(url) => {
                    url.query_pairs().for_each(|(param_name, _param_value)| {
                        all_params.push(param_name.to_string());
                    });
                    Ok(all_params)
                }
                None => Err("No url found").to_lua_err(),
            }
        });
        methods.add_method("urljoin", |_, this, new_path: String| {
            Ok(this.urljoin(new_path.as_str()))
        });

        // get_param - get single parameter value by name
        methods.add_method("get_param", |_, this, name: String| match &this.url {
            Some(url) => {
                for (k, v) in url.query_pairs() {
                    if k == name {
                        return Ok(Some(v.to_string()));
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        });

        // get_params - get all parameters as table
        methods.add_method("get_params", |lua, this, ()| {
            let table = lua.create_table()?;
            if let Some(url) = &this.url {
                for (k, v) in url.query_pairs() {
                    table.set(k.to_string(), v.to_string())?;
                }
            }
            Ok(table)
        });

        // has_param - check if parameter exists
        methods.add_method("has_param", |_, this, name: String| match &this.url {
            Some(url) => {
                for (k, _) in url.query_pairs() {
                    if k == name {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            None => Ok(false),
        });

        // host - get hostname
        methods.add_method("host", |_, this, ()| match &this.url {
            Some(url) => Ok(url.host_str().unwrap_or("").to_string()),
            None => Ok(String::new()),
        });

        // scheme - get URL scheme (http/https)
        methods.add_method("scheme", |_, this, ()| match &this.url {
            Some(url) => Ok(url.scheme().to_string()),
            None => Ok(String::new()),
        });

        // port - get port number (returns default port for scheme if not specified)
        methods.add_method("port", |_, this, ()| match &this.url {
            Some(url) => Ok(url.port_or_known_default().unwrap_or(0) as i32),
            None => Ok(0),
        });

        // set_path - modify URL path and return new URL string
        methods.add_method("set_path", |_, this, new_path: String| match &this.url {
            Some(url) => {
                let mut new_url = url.clone();
                new_url.set_path(&new_path);
                Ok(new_url.to_string())
            }
            None => Err("No url found".to_lua_err()),
        });
    }
}
