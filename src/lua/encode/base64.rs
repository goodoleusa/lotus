use base64::{engine::general_purpose::STANDARD, Engine};
use mlua::prelude::LuaResult;
use mlua::ExternalError;
use mlua::Lua;

pub fn base64_encode<'lua>(_: &'lua Lua, txt: String) -> LuaResult<String> {
    Ok(STANDARD.encode(txt.as_bytes()))
}

pub fn base64_decode<'lua>(_: &'lua Lua, txt: String) -> Result<Vec<u8>, mlua::Error> {
    match STANDARD.decode(txt.as_bytes()) {
        Ok(decoded_content) => Ok(decoded_content),
        Err(err) => Err(err.to_lua_err()),
    }
}
