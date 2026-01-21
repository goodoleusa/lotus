## Lotus Scripting Documentation

- [Starting 101](#starting-point)
- [Utility Classes](#utility-classes)
  - [Str - String Utilities](#str---string-utilities)
  - [Json - JSON Handling](#json---json-handling)
  - [Html - HTML/CSS Selectors](#html---htmlcss-selectors)
  - [Regex - Pattern Matching](#regex---pattern-matching)
- [Url Parsing](#url-parsing)
  - [URL Helper Methods](#url-helper-methods)
- [HTTP Requests](#http-requests)
  - [Response Helper Methods](#response-helper-methods)
- [Change the global request options](#change-the-global-request-options)
- [Fuzzing](#fuzzing)
- [Logging](#logging)
- [Modules](#modules)
- [Text Matching (Legacy)](#text-matching)
- [Reporting](#reporting)
- [Error Handling](#handle-connection-errors)
- [Custom input handler](#input-handling)
- [FAQ](#faq)

### Starting Point
Make the main() function globally accessible, and try the Lotus utilities to write a great script but make sure to set your Script type first
but first you've to define which scanning type you will do with your script,


| SCAN ID | INPUT TYPE                                | Example                                           | Access It           |
| ---     | ---                                       | ---                                               | ---                 |
| 1       | HOSTS                                     | `testphp.vulnweb.com`                             | `INPUT_DATA`        |
| 2       | FULL URL Including Parameters             | `http://testphp.vulnweb.com/artists.php?artist=1` | `HttpMessage:url()` |
| 3       | Passing URL Paths only without Parameters | `http://testphp.vulnweb.com/artists.php?artist=1` | `HttpMessage:url()` |
| 4       | Custom input handler                      | it can be anything but for example `123.1.2.3.5`  | `INPUT_DATA`        |


```lua 
-- hacking_script.lua

SCAN_TYPE = 2 -- Give me a full url with parameters

function main() 
    println("Hello World :D")
end
```
and then call it

```bash
$ echo "http://target.com" | lotus scan hacking_script.lua 
Hello World :D
```

> at the moment, lotus 0.5-beta is only sending http requests via one http library that means you cannot send a requests by using Socket or DNS, we're planning to add this in the upcoming version

---

## Chainable API (v0.6.0)

Lotus provides a powerful **fluent/chainable API** for string manipulation, HTML parsing, JSON handling, and more. Chain multiple operations together for clean, readable code.

### str() - Chainable Strings

```lua
-- Basic chaining
str("  hello world  "):trim():upper():value()
-- "HELLO WORLD"

-- Multiple transformations
str("hello"):upper():append("!"):prepend("["):value()
-- "[HELLO!]"

-- Split and process
str("a,b,c"):split(","):join(" | "):value()
-- "a | b | c"

-- Regex operations
str("abc123def456"):find_all("\\d+"):value()
-- {"123", "456"}

str("hello123world"):replace_regex("\\d+", "X"):value()
-- "helloXworld"

-- Encoding (all chainable)
str("<script>"):html_encode():value()           -- "&lt;script&gt;"
str("hello"):base64_encode():value()            -- "aGVsbG8="
str("hello world"):url_encode():value()         -- "hello%20world"

-- Predicates
str("test.php"):endswith(".php")                -- true
str("hello"):contains("ell")                    -- true
str("  "):is_empty()                            -- false

-- Print in chain (continues chain)
str("debug"):upper():print():append("!"):value()
-- prints "DEBUG", returns "DEBUG!"
```

**All str() methods:**
- Transforms: `upper()`, `lower()`, `trim()`, `ltrim()`, `rtrim()`, `reverse()`, `replace(from,to)`, `sub(start,end)`, `append(s)`, `prepend(s)`, `rep(n)`
- Regex: `match(pattern)`, `find(pattern)`, `find_all(pattern)`, `replace_regex(pattern,repl)`
- Encoding: `url_encode()`, `url_decode()`, `base64_encode()`, `base64_decode()`, `html_encode()`, `html_decode()`
- Predicates: `contains(s)`, `startswith(s)`, `endswith(s)`, `is_empty()`, `equals(s)`
- Output: `value()`, `val()`, `v()`, `len()`, `print()`
- Split: `split(delim)` -> returns chainable table

### html() - Chainable HTML Parser

```lua
-- Select and iterate
html(body):select("a.link"):each(function(el)
    local href = el:attr("href"):value()
    local text = el:text():value()
    println(href .. " -> " .. text)
end)

-- Get first match
local title = html(body):select_one("h1"):text():value()

-- Chain selections
html(body):select("div.content"):select("p"):first():text():value()

-- Get multiple attributes
html(body):select("a"):attr("href"):value()

-- XSS selector generation
html("<img src=x onerror=alert(1)>"):xss_selector():value()
-- 'img[src="x"][onerror="alert(1)"]'

-- Count matches
html(body):select("a"):len()
```

**All html() methods:**
- Selection: `select(css)`, `select_one(css)`, `first()`, `last()`, `get(idx)`
- Extract: `attr(name)`, `text()`, `xss_selector()`
- Output: `value()`, `list()`, `len()`
- Iterate: `each(fn)`

### json() - Chainable JSON Navigator

```lua
-- Parse and navigate with dot notation
json(body):get("data.users.0.name"):value()

-- Iterate arrays
json(body):get("users"):each(function(user)
    println(user:get("name"):value())
end)

-- Check existence
json(body):has("error")                         -- true/false
json(body):get("data"):is_null()                -- true/false

-- Get object keys
json(body):get("config"):keys():value()         -- {"key1", "key2"}

-- Pretty print
json(body):pretty():print()
```

**All json() methods:**
- Navigate: `get(key)`, `get("path.to.value")`, `has(key)`, `keys()`
- Type checks: `is_null()`, `is_string()`, `is_number()`, `is_bool()`, `is_array()`, `is_object()`
- Output: `value()`, `str()`, `pretty()`, `len()`, `print()`
- Iterate: `each(fn)`

### tbl() - Chainable Tables

```lua
-- Create from array
tbl({"a", "b", "c"}):join(", "):value()         -- "a, b, c"

-- Filter and map
tbl(payloads)
    :filter(function(p) return str(p):contains("<") end)
    :map(function(p) return str(p):upper():value() end)
    :value()

-- Sort and unique
tbl({"z", "a", "a", "m"}):unique():sort():value()
-- {"a", "m", "z"}

-- Find first match
tbl(items):find(function(x) return x == "target" end):value()

-- Any/All predicates
tbl({"1", "2", "3"}):all(function(x) return tonumber(x) end)  -- true
tbl({"a", "2", "c"}):any(function(x) return tonumber(x) end)  -- true
```

**All tbl() methods:**
- Access: `get(idx)`, `first()`, `last()`, `take(n)`, `skip(n)`
- Transform: `reverse()`, `sort()`, `unique()`, `join(delim)`
- Iterate: `map(fn)`, `filter(fn)`, `each(fn)`, `find(fn)`
- Predicates: `any(fn)`, `all(fn)`, `contains(s)`
- Output: `value()`, `val()`, `len()`, `print()`

### Global Functions

```lua
sleep(2)                    -- pause 2 seconds
sleep(0.5)                  -- pause 500ms
randstr(16):value()         -- random 16-char string (chainable)
```

---

### URL Parsing
* [Changing the URL Query](#changing-the-url-query)
* [Sending HTTP Requests](#http-requests)
* [Chaning the Default Connection options](#change-the-request)
* [Handling Connection Errors](#handle-connection-errors)

#### Changing the URL Query
It is possible to use the HttpMessage Lua Class to get your target URL, with this class you are able to perform the following:
- Get the target URL
```lua
-- echo "http://target.com/?is_admin=true" | lotus scan script.lua 
local target_url = HttpMessage:url()
-- http://target.com/?is_admin=true
```

- Getting all parameters in String
```lua
-- echo "http://target.com/?is_admin=true&year=2023" | lotus scan script.lua
local params = HttpMessage:param_str()
-- "is_admin=true&year=2023"
```
- Get iterator with all url query
```lua
-- echo "http://target.com/?is_admin=true&year=2023" | lotus scan script.lua 
local iter_params = HttpMessage:param_list()

for param_name, param_value in ipairs(iter_params) do 
    -- param_name: is_admin
    -- param_value: true
end
```
- Changing the value of custom Parameter
```lua
-- URL = https://target.com/users?name=Mike&age=20
local new_url = HttpMessage:param_set("age","23")
-- https://target.com/users?name=Mikehacker&age=2023
```

- Changing the value of all parameters
```lua
-- URL = https://target.com/users?name=Mike&age=20
local new_params = HttpMessage:param_set_all("<h1>",true) -- true = remove the parameter value
for param_name,param_value in ipairs(new_params) do 
    -- param_name: name
    -- param_value: <h1>
    -- continue ..
end
```

- Join URL Path for root path
```lua
make sure to make the global variable SCAN_TYPE value to 3 to make lotus pass the full path instead of parameters to avoid dups inputs
-- URL = https://target.com/users?name=Mike&age=20
local new_url = HttpMessage:urljoin("/admin/login?login=true")
-- URL = https://target.com/admin/login?login=true
Join URL Path for current path
-- make sure that your path doesn't starts with /
local new_url = pathjoin(HttpMessage:path(),"admin/login.php")
-- http://target.com/index.php/admin.login.php
```

#### URL Helper Methods
Additional methods for working with URLs:
- Get single parameter value
```lua
-- URL = https://target.com/?id=123&name=test
local id = HttpMessage:get_param("id")
-- "123"
```
- Get all parameters as table
```lua
-- URL = https://target.com/?id=123&name=test
local params = HttpMessage:get_params()
-- params.id = "123", params.name = "test"
```
- Check if parameter exists
```lua
if HttpMessage:has_param("id") then
    println("id parameter exists")
end
```
- Get hostname
```lua
-- URL = https://api.example.com/users
local host = HttpMessage:host()
-- "api.example.com"
```
- Get URL scheme
```lua
local scheme = HttpMessage:scheme()
-- "https"
```
- Get port number
```lua
-- URL = https://example.com:8443/
local port = HttpMessage:port()
-- 8443 (returns default port if not specified: 80 for http, 443 for https)
```
- Set URL path
```lua
-- URL = https://target.com/users?id=1
local new_url = HttpMessage:set_path("/admin")
-- "https://target.com/admin?id=1"
```

### HTTP Requests
Your lua script must call the HTTP lua class whose methods are assigned to the rust HTTP module in order to send HTTP requests
Send any method that you wish with a body and headers, but make sure that the headers are in Lua tables rather than strings
Sending normal GET request
Using the 'http:send()' function will permit you to send an HTTP request directly, but make sure you add the URL first since this field is required by the function, Keep in mind that `http:send` takes the connection options from the user options. If you need to change the connection options for your script, you can visit [#change the request](#change-the-request).

```lua
local resp = http:send{ url = "https://google.com" }
by adding this line you will call the https://google.com with GET method you will recive table with the response body/headers/url

local resp = http:send{ url = "https://google.com"}
println(resp.body) -- use println function to print message above the progress bar
for header_name,header_value in ipairs(resp.headers) do 
    println(string.format("%s: %s",header_name, header_value))
end
```
- Sending POST Requests
```lua
local headers = {}
headers["X-API"] = "RANDOM_DATA"
headers["Content-Type"] = "application/json"
local resp = http:send{ method = "POST", url = "http://target.com/api/users", body = '{"user_id":1}', headers = headers }
```

- Sending multipart
```lua
multipart = {} -- {param_name: content}
param_content = {}
param_content["content"] = "khaled" // parameter body [required]
param_content["content_type"] = "text/html" // parameter content-type [optional]
param_content["filename"] = "name.html" // filename [optional]
multipart["name"] = param_content
local resp = http:send{method="POST",url="http://google.com",multipart = multipart, timeout=10, headers=headers})
```
- Merge Headers (remove default headers if its has the same name of your headers)

```lua
http:merge_headers(true)

local headers = {}
headers["User-agent"] = "<img src=x onerror=alert()>"
local resp = http:send{url="http://google.com", headers=headers}
```

- Change redirects limit
```lua
http:send{url="http://google.com",redirects=5}
```

#### Response Helper Methods
The response object returned by `http:send` has several helper methods:
- Parse response body as JSON
```lua
local resp = http:send{url="https://api.example.com/users/1"}
local data = resp:json()
println(data.name) -- access JSON fields directly
```
- Check if response status is successful (2xx)
```lua
local resp = http:send{url="https://example.com"}
if resp:status_ok() then
    println("Request succeeded!")
end
```
- Check if header exists (case-insensitive)
```lua
local resp = http:send{url="https://example.com"}
if resp:has_header("Content-Type") then
    println("Has content type header")
end
```
- Get header value (case-insensitive)
```lua
local resp = http:send{url="https://example.com"}
local content_type = resp:get_header("content-type")
-- "text/html; charset=utf-8"
local location = resp:get_header("Location")
-- returns nil if header doesn't exist
```

### Change the global request options

You can change the default http connection options of your script
- Connection timeout
```lua
http:set_timeout(10) -- 10 secs
```
- limits of redirects
```lua
http:set_redirects(1) -- no redirects
http:set_redirects(2) -- only one redirect
```
- Custom Proxy
```lua
http:set_proxy("http://localhost:8080")
```
keep in mind this will only works in your script not in all scripts, so every time you call http:send function, the options that you changed will be called


### Input Handing
To handle input, create a new Lua script with a `parse_input` function. This function should take an input string and parse it according to your specific logic, then return a Lua table as output.

Here is an example implementation of the parse_input function:

like this
```lua
function parse_input(input)
    local output = {}
    output["hacker"] = 1,
    output["admin"] = 2,
    return output
end
```


Here is an example implementation of the parse_input function:

```lua
SCAN_TYPE = 4


function main()
    println(INPUT_DATA) -- LuaTable[hacker = 1]
end
```
Note that the main function in this example simply prints out the value of the hacker key in the parsed input table, but you can modify this function to suit your specific needs.


```bash
$ echo "hello" | lotus scan script.lua --input-handler input.lua 
```
### Handle Connection Errors
When using the "http:send" function, you might encounter a connections error because of the target response, so to ensure your script is not panicked, call the function within the protect function in the Lua language. This statement only returns a boolean value indicating whether the function has errors or not. For more information about pcall, please see the following link.
```lua
local func_status, resp = pcall(function () 
        return http:send("GET","http://0.0.0.0") -- request the localhost
        end)
if func_status == true then 
    -- True means no errors
    println("MAN WAKE UP I CAN ACCESS YOUR LOCAL NETWORK")
end
```
Also you can tell lotus about the error by adding a logging lines for it
```lua
if func_status == true then 
    -- True means no errors
    println("MAN WAKE UP I CAN ACCESS YOUR LOCAL NETWORK")
else 
    log_error(string.format("Connection Error: %s",func_status))
end
```

#### what if you want to check for custom error message ?
For example, if you have a Time-based Blind SQL Scanner, the only way to
determine whether a parameter is vulnerable is to set your Connection Timeout
to a value lower than the value for the SQL SLEEP Function Therefore, you must
verify whether the error was caused by a connection timeout or not
This can be accomplished by adding this function to your LUA script, and then sending the pcall error output to the function along with the error string message
```lua
function error_contains(error_obj, error_msg)
    -- ERR_STRING => Converting Error message from error type to string
    return str_contains(ERR_STRING(error_obj),error_msg)
end


function main() 
    local status, resp = pcall(function () 
        return http:send("GET","http://timeouthost")
    end)
    if status ~= true then 
        local timeout_err = error_contains(resp,"caused by: runtime error: timeout_error")
        if timeout_err == true then 
            println("TIMEOUT ERROR")
        end
    end
end
```
#### Connection ERROR Table

| Error        | Lua Code             |
| ---          | ----                 |
| Timeout      | `timeout_error`      |
| Connection   | `connection_error`   |
| Request Body | `request_body_error` |
| Decode       | `decode_error`       |
| External     | `external_error`     |







### Text Matching

> **Note**: The functions below are legacy APIs kept for backwards compatibility.
> For new scripts, prefer using the [Utility Classes](#utility-classes) (`Str`, `Json`, `Html`, `Regex`).

#### Legacy String Functions
```lua
-- These work but prefer Str class methods
str_contains("I use lua", "use")     -- true (use Str:contains instead)
str_startswith("I use lua", "I use") -- true (use Str:startswith instead)
str_endswith("test.php", ".php")     -- true (use Str:endswith instead)
str_split("a,b,c", ",")              -- table  (use Str:split instead)
str_trim("  hello  ")                -- "hello" (use Str:trim instead)
is_match("\\d+", "123")              -- true (use Regex:match instead)
random_string(16)                    -- random string (use Str:random instead)
```

#### Legacy JSON Functions
```lua
-- These work but prefer Json class methods
json_encode({name = "test"})         -- JSON string (use Json:encode instead)
json_decode('{"a":1}')               -- Lua table (use Json:decode instead)
```

#### Legacy HTML Functions
```lua
-- These work but prefer Html class methods
html_search(html, "h2#title")        -- table (use Html:select instead)
html_attr(element, "href")           -- string (use Html:attr instead)
html_text(element)                   -- string (use Html:text instead)
generate_css_selector(payload)       -- string (use Html:xss_selector instead)
```

#### ResponseMatcher (Advanced Matching)
For complex text matching with multiple conditions:
```lua
SCAN_TYPE = 2

function main()
	local match_one = {"test","Mike"}
	local match_all = {"Mike","true"}
	local BODY = '{"name":"Mike","is_admin":true}'
	-- match body with `or` conditions (returns true if ANY element matches)
	ResponseMatcher:match_body_once(BODY,match_one) -- true
	-- match body with `and` conditions (returns true if ALL elements match)
	ResponseMatcher:match_body(BODY,match_all) -- true
end
```



## Reporting

Lotus is giving you one simple way to report/save the output of your script, every time you run a script lotus would expect a list of findings in your report, it means you can include many finidings in the same report and the script as well so first you've to set the report information and after that call a global Lua Class called Reports

`Reports:add` accepts any value so feel free to add whatever you want in the report

```lua
local match = {}
match["123"]
match["456"]
Reports:add{
    url = "http://target.com",
    match = match
}
```
after that you will find the results in CLI and the json output (-o json)



### Logging
| Log Level | Lua Function |
| ---       | --           |
| INFO      | `log_info`   |
| DEBUG     | `log_debug`  |
| WARN      | `log_warn`   |
| ERROR     | `log_error`  |


```lua
local main()
    log_debug("Hello MOM :D")
end
```


```bash
$ echo "http://target.com"| lotus urls main.lua -o out.json --log log.txt
$ cat log.txt
[2023-02-28][14:40:09][lotus::cli::bar][INFO] URLS: 1
[2023-02-28][14:40:09][lotus::lua::parsing::files][DEBUG] READING "main.lua"
[2023-02-28][14:40:09][lotus][DEBUG] Running PATH scan 0
[2023-02-28][14:40:09][lotus::lua::parsing::files][DEBUG] READING "main.lua"
[2023-02-28][14:40:09][lotus][DEBUG] Running URL scan 1
[2023-02-28][14:40:09][lotus][DEBUG] Running main.lua script on http://target.com
[2023-02-28][14:40:09][lotus::lua::loader][DEBUG] Hello MOM :D
```



### Modules
While we strive to provide as many functions as possible, there may be cases where you require additional libraries for specific purposes. To address this, we have released packages on luarocks.org written in Rust for improved memory safety.

To use these packages, it is important to first install Rust from here. Once Rust is installed, you can search for packages released by the knas user on luarocks.org. Additionally, you can use any Lua modules written in different languages such as C.

### Fuzzing

lotus is focusing to make the fuzzing or multi-threading process easy and simple by providing two class to help in common fuzzing cases


the first one is  for parameter scanning that doesn't means this the can be used for Param Scanner this but the idea is this class has been created for that reason

##### ParamScan
this class takes one string with List, for the target parameter to scan and the payloads list, after that the ParamScan class will send the target parameter with every item in the payloads list to the target function
> target function is just lua function you create to so simple thing like sending http requests and return the response  

after sending it to the target function it will take the output of this function and then send it to the callback function

> Callback function is list the target function but for parsing 


in you callback function parse the target function output and see if this able is valid to save it in the report or not 

> FUZZ_WORKERS is lua varaible the value of --fuzz-workers option
```lua
SCAN_TYPE = 2

local function send_report(url,parameter,payload,matching_error)
    Reports:add {
        name = "Template Injection",
        link = "https://owasp.org/www-project-web-security-testing-guide/v41/4-Web_Application_Security_Testing/07-Input_Validation_Testing/18-Testing_for_Server_Side_Template_Injection",
        risk = "high",
        url = url,
        match = matching_error,
        param = parameter
    }
end

SSTI_PAYLOADS = {
    "lot{{2*2}}us",
    "lot<%= 2*2 %>us"
}

function scan_ssti(param_name,payload)
    local new_url = HttpMessage:setParam(param_name,payload)
    local resp_status,resp = pcall(function ()
        return http:send("GET",new_url) -- Sending a http request to the new url with GET Method
    end)
        if resp_status == true then
            local out = {}
            local body = resp.body -- Get the response body as string
            out["body"] = body
            out["url"] = resp.url
            out["param_name"] = param_name
            out["payload"] = payload
            return out
        end
end

function ssti_callback(data)
    if data == nil then
        return -- avoid nil cases
    end
    url = data["url"]
    body = data["body"]
    payload = data["payload"]
    param_name = data["param_name"]
    local match_status, match = pcall(function () 
        -- Matching with the response and the targeted regex
        -- we're using pcall here to avoid regex errors (and panic the code)
        return str_contains(body, "lot4us")
    end)
    if match_status == true then
        if match == true then
            send_report(url,param_name,payload,"lot4us")
            Reports:addVulnReport(VulnReport)
        end
    end
end

function main()
    for _,param in ipairs(HttpMessage:Params()) do
        ParamScan:start_scan()
        ParamScan:add_scan(param,SSTI_PAYLOADS, scan_ssti,ssti_callback, FUZZ_WORKERS)
    end
end
```

Basically, we are doing a for loop on all url parameters in the code above and
then creating a scanning thread with the target parameter, the SSTI_PAYLOAD
List, scan_ssti as the target function and ssti_callback as the callback
function, and FUZZ_WORKERS is a lua variable that gets its value from the
`--fuzz-workers` parameter (you can replace it with real number of you want) 

As part of the ssti_scan function, we change the parameter value to the SSTI
payload, and then send an HTTP request to it, and return a list with the
following components: body, url, payload, parameter name. 

ParamScan will then take the output of this function and pass it to the function callback
(ssti_callback). in the call callback function first lines it checks if the
function parameter value is nil (Null) or not because doing any match You may
set this option to prevent ParamScan from sending Nil to the call_back
functions

```lua
ParamScan:accept_nil(false) -- Dont pass any nil values
ParamScan:is_accept_nil() -- check if ParamScan is passing nil values or not
```
If you are scanning parameters, you do not need to call any of these functions since the default option is not to pass any null values to them
From anywhere in your script, you may call the ParamScan:stop_scan() function to stop the scanner and clear all futures
You can disable this option by using the ParamScan:start_scan() function
and if you want to check first if ParamScan is stopped or not you can use ParamScan:is_stop()

#### LuaThreader
this a simple class to do multi-threading, it only takes iterator and function to run 
```lua
SCAN_TYPE = 2

PAYLOADS = {
    "hello",
    'world'
}
function SCANNER(data)
    -- DO YOUR SCANNING LOGIC
end

function main()
    LuaThreader:run_scan(PAYLOADS,SCANNER,10) -- 10 = Number of workers
    -- LuaThreader:stop_scan() = stop the scan and dont accept any new futures
    -- LuaThreader:is_stop() = Check if LuaThreader is stopped or not
end
```
The LuaThreader class will open two threads in this example, one for the hello word and one for the world word
It is really as simple as that 



### Reading Files

- Reading files
```lua
local status, file = pcall(function()
    return readfile("/etc/passwd") 
end)
if status == true then 
    println(file)
end
```
- Path Join
```lua
pathjoin("/etc/","passwd") -- /etc/passwd
```
- Path Join in the script directory 
```lua
-- script dir /home/docker/scripts/main.lua
join_script_dir("payloads/sqli.txt")
-- /home/docker/scripts/payloads/sqli.txt
```
- Convert files to iterators by new lines
```lua
local status, lines = pcall(function()
    return readfile("/etc/passwd") 
end)
if status == true then 
    for word in line:gmatch("%w+") do 
        --
    end 
end
```
you can see the offical Lua IO Library for more informations  



### Encoding 

- Base64
```lua
base64encode("hello") -- aGVsbG8=
base64decode("aGVsbG8=") -- hello
```

- URL encoding
```lua
urlencode("Hello World") -- Hello%20World
urldecode("Hello%20World") -- Hello World
```

- HTML encoding
```lua
htmlencode("<script>alert()</script>") -- &lt;script&gt;alert()&lt;/script&gt;
htmldecode("&lt;script&gt;alert()&lt;/script&gt;") -- <script>alert()</script>
```


### FAQ

##### Comercial Use 
Thank you first for using lotus commercially
However, you should keep in mind that the Lotus Project is licensed under the GPLv2 license, which allows commercial use of the project, however it requires you to open a PR or inform the Lotus Project Team if you made any changes to the core code
Lotus is doing this because we want to ensure that everyone has access to all of its features
It does not mean that your lua scripts should be shared with others. We actually use BSD licenses for lua scripts, which allow you to hide your scripts according to your preferences

Would you like to discuss with the team the possibility of releasing Lotus in other license for you?
just send an email to knassar702@gmail.com
Feel free to send the same email if you need assistance with how to use Lotus effectively for your business
It would be great if you could join a meeting with the Lotus team and discuss this in more detail:)



##### I can't find the function that I need
you can download any library from https://luarocks.org/ and then import it in your script 
Or open an issue on our Github repository for the functionality you are missing
