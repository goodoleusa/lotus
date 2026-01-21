use crate::lua::{
    model::LuaRunTime,
    parsing::text::ResponseMatcher,
    threads::{LuaThreader, ParamScan},
};
use crate::utils::bar::GLOBAL_PROGRESS_BAR;
use mlua::{Function, UserData, Value};
use rand::Rng;
use regex::Regex as RustRegex;
use scraper::{Html as ScraperHtml, Selector};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

macro_rules! set_global_function {
    ($lua:expr, $name:expr, $func:expr) => {
        $lua.globals().set($name, $func).unwrap();
    };
}

// ============================================================================
// LotusStr - Chainable String Wrapper
// ============================================================================
// Usage: str("hello"):upper():trim():value()

#[derive(Clone, Debug)]
pub struct LotusStr {
    pub data: String,
}

impl LotusStr {
    pub fn new(s: String) -> Self {
        Self { data: s }
    }
}

impl UserData for LotusStr {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Get the raw value
        methods.add_method("value", |_, this, ()| Ok(this.data.clone()));
        methods.add_method("val", |_, this, ()| Ok(this.data.clone()));
        methods.add_method("v", |_, this, ()| Ok(this.data.clone()));

        // Length
        methods.add_method("len", |_, this, ()| Ok(this.data.len()));

        // Chainable transformations
        methods.add_method("upper", |_, this, ()| {
            Ok(LotusStr::new(this.data.to_uppercase()))
        });

        methods.add_method("lower", |_, this, ()| {
            Ok(LotusStr::new(this.data.to_lowercase()))
        });

        methods.add_method("trim", |_, this, ()| {
            Ok(LotusStr::new(this.data.trim().to_string()))
        });

        methods.add_method("ltrim", |_, this, ()| {
            Ok(LotusStr::new(this.data.trim_start().to_string()))
        });

        methods.add_method("rtrim", |_, this, ()| {
            Ok(LotusStr::new(this.data.trim_end().to_string()))
        });

        methods.add_method("reverse", |_, this, ()| {
            Ok(LotusStr::new(this.data.chars().rev().collect()))
        });

        methods.add_method("replace", |_, this, (from, to): (String, String)| {
            Ok(LotusStr::new(this.data.replace(&from, &to)))
        });

        // Substring (1-indexed like Lua)
        methods.add_method("sub", |_, this, (start, end): (usize, Option<usize>)| {
            let start_idx = start.saturating_sub(1);
            let end_idx = end.unwrap_or(this.data.len());
            let result: String = this.data.chars().skip(start_idx).take(end_idx - start_idx).collect();
            Ok(LotusStr::new(result))
        });

        // Append
        methods.add_method("append", |_, this, s: String| {
            Ok(LotusStr::new(format!("{}{}", this.data, s)))
        });

        methods.add_method("prepend", |_, this, s: String| {
            Ok(LotusStr::new(format!("{}{}", s, this.data)))
        });

        // Repeat
        methods.add_method("rep", |_, this, n: usize| {
            Ok(LotusStr::new(this.data.repeat(n)))
        });

        // Predicates (return bool, break chain)
        methods.add_method("contains", |_, this, s: String| {
            Ok(this.data.contains(&s))
        });

        methods.add_method("startswith", |_, this, s: String| {
            Ok(this.data.starts_with(&s))
        });

        methods.add_method("endswith", |_, this, s: String| {
            Ok(this.data.ends_with(&s))
        });

        methods.add_method("is_empty", |_, this, ()| {
            Ok(this.data.is_empty())
        });

        methods.add_method("equals", |_, this, s: String| {
            Ok(this.data == s)
        });

        // Split returns LotusTable for continued chaining
        methods.add_method("split", |_, this, delim: String| {
            let parts: Vec<String> = this.data.split(&delim).map(|s| s.to_string()).collect();
            Ok(LotusTable::new(parts))
        });

        // Regex operations
        methods.add_method("match", |_, this, pattern: String| {
            match RustRegex::new(&pattern) {
                Ok(re) => Ok(re.is_match(&this.data)),
                Err(_) => Ok(false),
            }
        });

        methods.add_method("find", |_, this, pattern: String| {
            match RustRegex::new(&pattern) {
                Ok(re) => {
                    if let Some(m) = re.find(&this.data) {
                        Ok(Some(LotusStr::new(m.as_str().to_string())))
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => Err(mlua::Error::RuntimeError(format!("Regex error: {}", e))),
            }
        });

        methods.add_method("find_all", |_, this, pattern: String| {
            match RustRegex::new(&pattern) {
                Ok(re) => {
                    let matches: Vec<String> = re.find_iter(&this.data)
                        .map(|m| m.as_str().to_string())
                        .collect();
                    Ok(LotusTable::new(matches))
                }
                Err(e) => Err(mlua::Error::RuntimeError(format!("Regex error: {}", e))),
            }
        });

        methods.add_method("replace_regex", |_, this, (pattern, replacement): (String, String)| {
            match RustRegex::new(&pattern) {
                Ok(re) => Ok(LotusStr::new(re.replace_all(&this.data, replacement.as_str()).to_string())),
                Err(e) => Err(mlua::Error::RuntimeError(format!("Regex error: {}", e))),
            }
        });

        // Encoding
        methods.add_method("url_encode", |_, this, ()| {
            Ok(LotusStr::new(urlencoding::encode(&this.data).to_string()))
        });

        methods.add_method("url_decode", |_, this, ()| {
            match urlencoding::decode(&this.data) {
                Ok(s) => Ok(LotusStr::new(s.to_string())),
                Err(_) => Ok(LotusStr::new(this.data.clone())),
            }
        });

        methods.add_method("base64_encode", |_, this, ()| {
            use base64::{Engine, engine::general_purpose::STANDARD};
            Ok(LotusStr::new(STANDARD.encode(this.data.as_bytes())))
        });

        methods.add_method("base64_decode", |_, this, ()| {
            use base64::{Engine, engine::general_purpose::STANDARD};
            match STANDARD.decode(this.data.as_bytes()) {
                Ok(bytes) => Ok(LotusStr::new(String::from_utf8_lossy(&bytes).into_owned())),
                Err(_) => Ok(LotusStr::new(this.data.clone())),
            }
        });

        methods.add_method("html_encode", |_, this, ()| {
            Ok(LotusStr::new(html_escape::encode_text(&this.data).to_string()))
        });

        methods.add_method("html_decode", |_, this, ()| {
            Ok(LotusStr::new(html_escape::decode_html_entities(&this.data).to_string()))
        });

        // Debug/print
        methods.add_method("print", |_, this, ()| {
            GLOBAL_PROGRESS_BAR.lock().unwrap().clone().unwrap().println(&this.data);
            Ok(LotusStr::new(this.data.clone()))
        });

        // Metamethod for tostring
        methods.add_meta_method("__tostring", |_, this, ()| Ok(this.data.clone()));
        methods.add_meta_method("__len", |_, this, ()| Ok(this.data.len()));
        methods.add_meta_method("__concat", |_, this, other: String| {
            Ok(LotusStr::new(format!("{}{}", this.data, other)))
        });
    }
}

// ============================================================================
// LotusTable - Chainable Array/Table Wrapper
// ============================================================================
// Usage: str("a,b,c"):split(","):map(fn):filter(fn):join("-")

#[derive(Clone, Debug)]
pub struct LotusTable {
    pub data: Vec<String>,
}

impl LotusTable {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl UserData for LotusTable {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Get raw value as Lua table
        methods.add_method("value", |lua, this, ()| {
            let table = lua.create_table()?;
            for (i, item) in this.data.iter().enumerate() {
                table.set(i + 1, item.clone())?;
            }
            Ok(table)
        });

        methods.add_method("val", |lua, this, ()| {
            let table = lua.create_table()?;
            for (i, item) in this.data.iter().enumerate() {
                table.set(i + 1, item.clone())?;
            }
            Ok(table)
        });

        // Length
        methods.add_method("len", |_, this, ()| Ok(this.data.len()));

        // Get item by index (1-indexed)
        methods.add_method("get", |_, this, idx: usize| {
            if idx > 0 && idx <= this.data.len() {
                Ok(Some(LotusStr::new(this.data[idx - 1].clone())))
            } else {
                Ok(None)
            }
        });

        // First/Last
        methods.add_method("first", |_, this, ()| {
            Ok(this.data.first().map(|s| LotusStr::new(s.clone())))
        });

        methods.add_method("last", |_, this, ()| {
            Ok(this.data.last().map(|s| LotusStr::new(s.clone())))
        });

        // Join back to string
        methods.add_method("join", |_, this, delim: String| {
            Ok(LotusStr::new(this.data.join(&delim)))
        });

        // Reverse
        methods.add_method("reverse", |_, this, ()| {
            let mut reversed = this.data.clone();
            reversed.reverse();
            Ok(LotusTable::new(reversed))
        });

        // Sort
        methods.add_method("sort", |_, this, ()| {
            let mut sorted = this.data.clone();
            sorted.sort();
            Ok(LotusTable::new(sorted))
        });

        // Unique
        methods.add_method("unique", |_, this, ()| {
            let mut unique: Vec<String> = Vec::new();
            for item in &this.data {
                if !unique.contains(item) {
                    unique.push(item.clone());
                }
            }
            Ok(LotusTable::new(unique))
        });

        // Take first N
        methods.add_method("take", |_, this, n: usize| {
            Ok(LotusTable::new(this.data.iter().take(n).cloned().collect()))
        });

        // Skip first N
        methods.add_method("skip", |_, this, n: usize| {
            Ok(LotusTable::new(this.data.iter().skip(n).cloned().collect()))
        });

        // Contains
        methods.add_method("contains", |_, this, s: String| {
            Ok(this.data.contains(&s))
        });

        // Map with function
        methods.add_method("map", |_, this, func: Function| {
            let mut result = Vec::new();
            for item in &this.data {
                let mapped: String = func.call(item.clone())?;
                result.push(mapped);
            }
            Ok(LotusTable::new(result))
        });

        // Filter with function
        methods.add_method("filter", |_, this, func: Function| {
            let mut result = Vec::new();
            for item in &this.data {
                let keep: bool = func.call(item.clone())?;
                if keep {
                    result.push(item.clone());
                }
            }
            Ok(LotusTable::new(result))
        });

        // Each - iterate and call function (returns self for chaining)
        methods.add_method("each", |_, this, func: Function| {
            for item in &this.data {
                func.call::<_, ()>(item.clone())?;
            }
            Ok(LotusTable::new(this.data.clone()))
        });

        // Find first matching
        methods.add_method("find", |_, this, func: Function| {
            for item in &this.data {
                let matches: bool = func.call(item.clone())?;
                if matches {
                    return Ok(Some(LotusStr::new(item.clone())));
                }
            }
            Ok(None)
        });

        // Any/All
        methods.add_method("any", |_, this, func: Function| {
            for item in &this.data {
                let matches: bool = func.call(item.clone())?;
                if matches {
                    return Ok(true);
                }
            }
            Ok(false)
        });

        methods.add_method("all", |_, this, func: Function| {
            for item in &this.data {
                let matches: bool = func.call(item.clone())?;
                if !matches {
                    return Ok(false);
                }
            }
            Ok(true)
        });

        // Debug print all
        methods.add_method("print", |_, this, ()| {
            for item in &this.data {
                GLOBAL_PROGRESS_BAR.lock().unwrap().clone().unwrap().println(item);
            }
            Ok(LotusTable::new(this.data.clone()))
        });

        methods.add_meta_method("__len", |_, this, ()| Ok(this.data.len()));
    }
}

// ============================================================================
// LotusHtml - Chainable HTML Parser
// ============================================================================
// Usage: html(body):select("a.link"):attr("href"):value()

#[derive(Clone, Debug)]
pub struct LotusHtml {
    pub data: String,
    pub elements: Vec<String>,
}

impl LotusHtml {
    pub fn new(html: String) -> Self {
        Self { data: html.clone(), elements: vec![html] }
    }

    pub fn from_elements(elements: Vec<String>) -> Self {
        Self { data: elements.join(""), elements }
    }
}

impl UserData for LotusHtml {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Get raw value
        methods.add_method("value", |_, this, ()| {
            if this.elements.len() == 1 {
                Ok(this.elements[0].clone())
            } else {
                Ok(this.elements.join("\n"))
            }
        });

        // Get as table
        methods.add_method("list", |_, this, ()| {
            Ok(LotusTable::new(this.elements.clone()))
        });

        // Count matches
        methods.add_method("len", |_, this, ()| Ok(this.elements.len()));

        // CSS selector
        methods.add_method("select", |_, this, selector: String| {
            let mut all_matches = Vec::new();
            for html in &this.elements {
                let document = ScraperHtml::parse_document(html);
                match Selector::parse(&selector) {
                    Ok(sel) => {
                        for element in document.select(&sel) {
                            all_matches.push(element.html());
                        }
                    }
                    Err(_) => {}
                }
            }
            Ok(LotusHtml::from_elements(all_matches))
        });

        // Select first only
        methods.add_method("select_one", |_, this, selector: String| {
            for html in &this.elements {
                let document = ScraperHtml::parse_document(html);
                if let Ok(sel) = Selector::parse(&selector) {
                    if let Some(element) = document.select(&sel).next() {
                        return Ok(Some(LotusHtml::new(element.html())));
                    }
                }
            }
            Ok(None)
        });

        // Get attribute
        methods.add_method("attr", |_, this, attr_name: String| {
            let mut attrs = Vec::new();
            for html in &this.elements {
                let fragment = ScraperHtml::parse_fragment(html);
                if let Some(root) = fragment.root_element().first_child() {
                    if let Some(element) = root.value().as_element() {
                        if let Some(attr_value) = element.attr(&attr_name) {
                            attrs.push(attr_value.to_string());
                        }
                    }
                }
            }
            if attrs.len() == 1 {
                Ok(Some(LotusStr::new(attrs[0].clone())))
            } else if attrs.is_empty() {
                Ok(None)
            } else {
                Ok(Some(LotusStr::new(attrs.join("\n"))))
            }
        });

        // Get text content
        methods.add_method("text", |_, this, ()| {
            let mut texts = Vec::new();
            for html in &this.elements {
                let fragment = ScraperHtml::parse_fragment(html);
                let text: String = fragment.root_element().text().collect();
                texts.push(text.trim().to_string());
            }
            if texts.len() == 1 {
                Ok(LotusStr::new(texts[0].clone()))
            } else {
                Ok(LotusStr::new(texts.join("\n")))
            }
        });

        // Get first element
        methods.add_method("first", |_, this, ()| {
            Ok(this.elements.first().map(|s| LotusHtml::new(s.clone())))
        });

        // Get last element
        methods.add_method("last", |_, this, ()| {
            Ok(this.elements.last().map(|s| LotusHtml::new(s.clone())))
        });

        // Get by index
        methods.add_method("get", |_, this, idx: usize| {
            if idx > 0 && idx <= this.elements.len() {
                Ok(Some(LotusHtml::new(this.elements[idx - 1].clone())))
            } else {
                Ok(None)
            }
        });

        // XSS selector generation
        methods.add_method("xss_selector", |_, this, ()| {
            let fragment = ScraperHtml::parse_fragment(&this.data);
            if let Some(root) = fragment.root_element().first_child() {
                if let Some(element) = root.value().as_element() {
                    let tag_name = element.name();
                    let mut selector_parts = vec![tag_name.to_string()];
                    for (name, value) in element.attrs() {
                        if name != "src" || !value.is_empty() {
                            selector_parts.push(format!("[{}=\"{}\"]", name, value));
                        }
                    }
                    return Ok(LotusStr::new(selector_parts.join("")));
                }
            }
            Ok(LotusStr::new(String::new()))
        });

        // Iterate elements
        methods.add_method("each", |_, this, func: Function| {
            for item in &this.elements {
                func.call::<_, ()>(LotusHtml::new(item.clone()))?;
            }
            Ok(LotusHtml::from_elements(this.elements.clone()))
        });
    }
}

// ============================================================================
// LotusJson - Chainable JSON Handler
// ============================================================================
// Usage: json(body):get("data.users"):get(1):get("name"):value()

#[derive(Clone, Debug)]
pub struct LotusJson {
    pub data: serde_json::Value,
}

impl LotusJson {
    pub fn new(val: serde_json::Value) -> Self {
        Self { data: val }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match serde_json::from_str(s) {
            Ok(v) => Ok(Self::new(v)),
            Err(e) => Err(format!("JSON parse error: {}", e)),
        }
    }
}

impl UserData for LotusJson {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // Get raw value
        methods.add_method("value", |lua, this, ()| {
            json_to_lua(lua, &this.data)
        });

        // Get as string
        methods.add_method("str", |_, this, ()| {
            Ok(LotusStr::new(serde_json::to_string(&this.data).unwrap_or_default()))
        });

        // Pretty print
        methods.add_method("pretty", |_, this, ()| {
            Ok(LotusStr::new(serde_json::to_string_pretty(&this.data).unwrap_or_default()))
        });

        // Get nested value by key or path (supports "data.users.0.name")
        methods.add_method("get", |_, this, key: Value| {
            let result = match key {
                Value::String(s) => {
                    let path = s.to_str().unwrap_or("");
                    let mut current = &this.data;
                    for part in path.split('.') {
                        if let Ok(idx) = part.parse::<usize>() {
                            current = &current[idx];
                        } else {
                            current = &current[part];
                        }
                    }
                    current.clone()
                }
                Value::Integer(i) => this.data[i as usize].clone(),
                _ => serde_json::Value::Null,
            };
            Ok(LotusJson::new(result))
        });

        // Check if key exists
        methods.add_method("has", |_, this, key: String| {
            Ok(!this.data[&key].is_null())
        });

        // Type checks
        methods.add_method("is_null", |_, this, ()| Ok(this.data.is_null()));
        methods.add_method("is_string", |_, this, ()| Ok(this.data.is_string()));
        methods.add_method("is_number", |_, this, ()| Ok(this.data.is_number()));
        methods.add_method("is_bool", |_, this, ()| Ok(this.data.is_boolean()));
        methods.add_method("is_array", |_, this, ()| Ok(this.data.is_array()));
        methods.add_method("is_object", |_, this, ()| Ok(this.data.is_object()));

        // Array length
        methods.add_method("len", |_, this, ()| {
            Ok(this.data.as_array().map(|a| a.len()).unwrap_or(0))
        });

        // Keys (for objects)
        methods.add_method("keys", |_, this, ()| {
            if let Some(obj) = this.data.as_object() {
                let keys: Vec<String> = obj.keys().cloned().collect();
                Ok(LotusTable::new(keys))
            } else {
                Ok(LotusTable::new(vec![]))
            }
        });

        // Iterate array
        methods.add_method("each", |_, this, func: Function| {
            if let Some(arr) = this.data.as_array() {
                for item in arr {
                    func.call::<_, ()>(LotusJson::new(item.clone()))?;
                }
            }
            Ok(LotusJson::new(this.data.clone()))
        });

        // Print for debugging
        methods.add_method("print", |_, this, ()| {
            let s = serde_json::to_string_pretty(&this.data).unwrap_or_default();
            GLOBAL_PROGRESS_BAR.lock().unwrap().clone().unwrap().println(&s);
            Ok(LotusJson::new(this.data.clone()))
        });

        methods.add_meta_method("__tostring", |_, this, ()| {
            Ok(serde_json::to_string(&this.data).unwrap_or_default())
        });
    }
}

// ============================================================================
// UtilsEXT trait implementation
// ============================================================================

pub trait UtilsEXT {
    fn add_threadsfunc(&self);
    fn add_matchingfunc(&self);
    fn add_printfunc(&self);
}

impl UtilsEXT for LuaRunTime<'_> {
    fn add_printfunc(&self) {
        set_global_function!(
            self.lua,
            "join_script_dir",
            self.lua
                .create_function(|c_lua, new_path: String| {
                    let script_path = c_lua.globals().get::<_, String>("SCRIPT_PATH").unwrap();
                    let the_path = Path::new(&script_path);
                    Ok(the_path
                        .parent()
                        .unwrap()
                        .join(new_path)
                        .to_str()
                        .unwrap()
                        .to_string())
                })
                .unwrap()
        );

        macro_rules! log_function {
            ($name:expr, $level:ident) => {{
                let log_func = self
                    .lua
                    .create_function(move |_, log_msg: String| {
                        log::$level!("{}", log_msg);
                        Ok(())
                    })
                    .unwrap();
                self.lua.globals().set($name, log_func).unwrap();
            }};
        }

        log_function!("log_info", info);
        log_function!("log_warn", warn);
        log_function!("log_debug", debug);
        log_function!("log_error", error);

        set_global_function!(
            self.lua,
            "println",
            self.lua
                .create_function(move |_, msg: String| {
                    GLOBAL_PROGRESS_BAR
                        .lock()
                        .unwrap()
                        .clone()
                        .unwrap()
                        .println(msg);
                    Ok(())
                })
                .unwrap()
        );
    }

    fn add_matchingfunc(&self) {
        // Legacy Matcher for backwards compatibility
        set_global_function!(
            self.lua,
            "Matcher",
            ResponseMatcher {
                ignore_whitespace: false,
                case_insensitive: false,
                multi_line: false,
                octal: true,
                unicode: true,
                dot_matches_new_line: false,
            }
        );

        // ====================================================
        // NEW CHAINABLE API - Factory Functions
        // ====================================================

        // str("hello") -> LotusStr
        set_global_function!(
            self.lua,
            "str",
            self.lua.create_function(|_, s: String| Ok(LotusStr::new(s))).unwrap()
        );

        // html("<div>...</div>") -> LotusHtml
        set_global_function!(
            self.lua,
            "html",
            self.lua.create_function(|_, s: String| Ok(LotusHtml::new(s))).unwrap()
        );

        // json('{"a":1}') -> LotusJson
        set_global_function!(
            self.lua,
            "json",
            self.lua.create_function(|_, s: String| {
                match LotusJson::from_str(&s) {
                    Ok(j) => Ok(j),
                    Err(e) => Err(mlua::Error::RuntimeError(e)),
                }
            }).unwrap()
        );

        // tbl({"a", "b", "c"}) -> LotusTable
        set_global_function!(
            self.lua,
            "tbl",
            self.lua.create_function(|_, t: Vec<String>| Ok(LotusTable::new(t))).unwrap()
        );

        // randstr(16) -> random string
        set_global_function!(
            self.lua,
            "randstr",
            self.lua.create_function(|_, len: usize| {
                let s: String = rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(len)
                    .map(char::from)
                    .collect();
                Ok(LotusStr::new(s))
            }).unwrap()
        );

        // sleep(secs)
        set_global_function!(
            self.lua,
            "sleep",
            self.lua.create_function(|_, secs: f64| {
                std::thread::sleep(Duration::from_secs_f64(secs));
                Ok(())
            }).unwrap()
        );

        // ====================================================
        // LEGACY API - Backwards Compatibility
        // ====================================================

        macro_rules! string_function {
            ($name:expr, $method:ident) => {{
                set_global_function!(
                    self.lua,
                    $name,
                    self.lua
                        .create_function(|_, (str_one, str_two): (String, String)| {
                            Ok(str_one.$method(&str_two))
                        })
                        .unwrap()
                );
            }};
        }

        string_function!("str_startswith", starts_with);
        string_function!("str_contains", contains);
        string_function!("str_endswith", ends_with);

        set_global_function!(
            self.lua,
            "is_match",
            self.lua.create_function(|_, (pattern, text): (String, String)| {
                match RustRegex::new(&pattern) {
                    Ok(re) => Ok(re.is_match(&text)),
                    Err(_) => Ok(false),
                }
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "str_split",
            self.lua.create_function(|lua, (text, delimiter): (String, String)| {
                let parts: Vec<&str> = text.split(&delimiter).collect();
                let table = lua.create_table()?;
                for (i, part) in parts.iter().enumerate() {
                    table.set(i + 1, *part)?;
                }
                Ok(table)
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "str_trim",
            self.lua.create_function(|_, text: String| Ok(text.trim().to_string())).unwrap()
        );

        set_global_function!(
            self.lua,
            "json_encode",
            self.lua.create_function(|_, value: Value| {
                let json_value = lua_to_json(value)?;
                match serde_json::to_string(&json_value) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(mlua::Error::RuntimeError(format!("JSON encode error: {}", e))),
                }
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "json_decode",
            self.lua.create_function(|lua, json_str: String| {
                match serde_json::from_str::<serde_json::Value>(&json_str) {
                    Ok(json_value) => json_to_lua(lua, &json_value),
                    Err(e) => Err(mlua::Error::RuntimeError(format!("JSON decode error: {}", e))),
                }
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "html_search",
            self.lua.create_function(|lua, (html_content, css_selector): (String, String)| {
                let document = ScraperHtml::parse_document(&html_content);
                match Selector::parse(&css_selector) {
                    Ok(selector) => {
                        let table = lua.create_table()?;
                        for (i, element) in document.select(&selector).enumerate() {
                            table.set(i + 1, element.html())?;
                        }
                        Ok(table)
                    }
                    Err(e) => Err(mlua::Error::RuntimeError(format!("CSS selector error: {:?}", e))),
                }
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "html_attr",
            self.lua.create_function(|_, (html_element, attr_name): (String, String)| {
                let fragment = ScraperHtml::parse_fragment(&html_element);
                if let Some(root) = fragment.root_element().first_child() {
                    if let Some(element) = root.value().as_element() {
                        if let Some(attr_value) = element.attr(&attr_name) {
                            return Ok(Some(attr_value.to_string()));
                        }
                    }
                }
                Ok(None)
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "html_text",
            self.lua.create_function(|_, html_element: String| {
                let fragment = ScraperHtml::parse_fragment(&html_element);
                let text: String = fragment.root_element().text().collect();
                Ok(text.trim().to_string())
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "generate_css_selector",
            self.lua.create_function(|_, payload: String| {
                let fragment = ScraperHtml::parse_fragment(&payload);
                if let Some(root) = fragment.root_element().first_child() {
                    if let Some(element) = root.value().as_element() {
                        let tag_name = element.name();
                        let mut selector_parts = vec![tag_name.to_string()];
                        for (name, value) in element.attrs() {
                            if name != "src" || !value.is_empty() {
                                selector_parts.push(format!("[{}=\"{}\"]", name, value));
                            }
                        }
                        return Ok(selector_parts.join(""));
                    }
                }
                Ok(String::new())
            }).unwrap()
        );

        set_global_function!(
            self.lua,
            "random_string",
            self.lua.create_function(|_, len: usize| {
                let s: String = rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(len)
                    .map(char::from)
                    .collect();
                Ok(s)
            }).unwrap()
        );
    }

    fn add_threadsfunc(&self) {
        set_global_function!(
            self.lua,
            "ParamScan",
            ParamScan {
                finds: Arc::new(Mutex::new(false)),
                accept_nil: Arc::new(Mutex::new(false)),
            }
        );

        set_global_function!(
            self.lua,
            "LuaThreader",
            LuaThreader {
                stop: Arc::new(Mutex::new(false)),
            }
        );
    }
}

// ============================================================================
// JSON conversion helpers
// ============================================================================

fn lua_to_json(value: Value) -> Result<serde_json::Value, mlua::Error> {
    match value {
        Value::Nil => Ok(serde_json::Value::Null),
        Value::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        Value::Integer(i) => Ok(serde_json::Value::Number(i.into())),
        Value::Number(n) => {
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(serde_json::Value::Number(num))
            } else {
                Err(mlua::Error::RuntimeError("Invalid number for JSON".to_string()))
            }
        }
        Value::String(s) => Ok(serde_json::Value::String(s.to_str()?.to_string())),
        Value::Table(table) => {
            let mut is_array = true;
            let mut max_index = 0;
            for pair in table.clone().pairs::<Value, Value>() {
                let (key, _) = pair?;
                match key {
                    Value::Integer(i) if i > 0 => {
                        if i > max_index { max_index = i; }
                    }
                    _ => { is_array = false; break; }
                }
            }

            if is_array && max_index > 0 {
                let mut arr = Vec::new();
                for i in 1..=max_index {
                    let val: Value = table.get(i)?;
                    arr.push(lua_to_json(val)?);
                }
                Ok(serde_json::Value::Array(arr))
            } else {
                let mut map = serde_json::Map::new();
                for pair in table.pairs::<Value, Value>() {
                    let (key, val) = pair?;
                    let key_str = match key {
                        Value::String(s) => s.to_str()?.to_string(),
                        Value::Integer(i) => i.to_string(),
                        Value::Number(n) => n.to_string(),
                        _ => return Err(mlua::Error::RuntimeError("JSON object keys must be strings".to_string())),
                    };
                    map.insert(key_str, lua_to_json(val)?);
                }
                Ok(serde_json::Value::Object(map))
            }
        }
        _ => Err(mlua::Error::RuntimeError("Cannot convert value to JSON".to_string())),
    }
}

fn json_to_lua<'lua>(lua: &'lua mlua::Lua, value: &serde_json::Value) -> Result<Value<'lua>, mlua::Error> {
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
