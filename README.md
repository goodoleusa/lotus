
<p align="center">
<img src="https://raw.githubusercontent.com/BugBlocker/lotus/master/logo/lotus_logo.png" width="370px" alt="Lotus Logo">
</p>

# Lotus: Dynamic Application Security Testing (DAST) Automation System

**Lotus** is an advanced automation system for Dynamic Application Security Testing (DAST). It simplifies web security scripting by offering a powerful Lua API that automates security processes in just a few lines of code. With Lotus, web security testing becomes more efficient, ensuring no critical security steps are missed.

Our mission is to streamline the security testing workflow, providing robust libraries and functions to automate web security processes with speed and precision.

## Table of Contents
- [Why Choose Lotus?](#why-choose-lotus)
- [Features of Lotus Scripts](#features-of-lotus-scripts)
- [Installation Guide](#rocket-installation-guide)
- [Documentation](#book-documentation)

## Why Choose Lotus?

### 1. Comprehensive Lua API
Lotus delivers a powerful Lua API packed with numerous functions designed to automate complex web security testing scenarios. With Lotus, automating security tasks becomes efficient, allowing you to focus on addressing vulnerabilities.

### 2. Optimized for Performance
Lotus is engineered for speed and accuracy, delivering top-tier performance in web security testing. This ensures faster vulnerability identification without the need for manual intervention.

### 3. Flexible and Customizable
Lotus offers unmatched flexibility with its customizable reporting, HTTP request handling, and input validation. Its API includes functions for regex matching and data validation, enabling precise detection of vulnerabilities.

### 4. Active and Ongoing Development
As an open-source project, Lotus is actively maintained and regularly updated. You can trust in continuous support and improvements to meet the evolving demands of web security testing.

### 5. Quick and Easy Installation
Installing Lotus is a simple process, whether you choose to compile from source or download pre-built binaries. Full documentation is provided to guide you through installation and usage.

### 6. Community Collaboration
We encourage community contributions! Share feedback, report issues, or suggest improvements through our [GitHub Issues](https://github.com/BugBlocker/lotus/issues) page, or connect with the community on our [Discord server](https://discord.gg/nBYDPTzjSq).

## Features of Lotus Scripts

Lotus scripts provide a variety of powerful capabilities for web security testing, including:

### New in v0.6.0: Chainable Fluent API

Write expressive, readable security scripts with method chaining:

```lua
-- String operations with chaining
str("  HELLO  "):trim():lower():replace("hello", "world"):value()
-- "world"

-- Split, filter, and transform
str("admin,user,guest"):split(",")
    :filter(function(x) return x ~= "guest" end)
    :join("|"):value()
-- "admin|user"

-- HTML parsing with CSS selectors
html(body):select("a.external"):each(function(el)
    println(el:attr("href"):value())
end)

-- JSON navigation with dot notation
json(resp.body):get("data.users.0.name"):value()

-- Built-in encoding
str(payload):url_encode():base64_encode():value()
```

### Core Capabilities

- **`str()`:** Chainable strings - trim, split, replace, regex, encoding
- **`html()`:** CSS selectors with chaining - select, attr, text, xss_selector
- **`json()`:** Navigate JSON with dot notation - get, keys, each
- **`tbl()`:** Chainable arrays - map, filter, sort, unique
- **Response Helpers:** `resp:json()`, `resp:status_ok()`, `resp:get_header()`
- **URL Helpers:** `HttpMessage:get_param()`, `host()`, `scheme()`, `port()`
- **Multi-Threading:** Two threading managers for optimal scanning
- **HTTP Requests:** All methods, custom headers, multipart support
- **Encoding:** Base64, URL, HTML - all chainable

Additionally, with Lua libraries available on LuaRocks, you can further extend Lotus's capabilities for your security testing needs.

## :rocket: Installation Guide

Follow these steps to install Lotus from the source code:

```bash
$ apt install libssl-dev pkg-config gcc git lua53 liblua5.3-0 liblua5.3-dev -y
$ cargo install --git=https://github.com/BugBlocker/lotus/
```

Ensure you have the `openssl-dev` package installed before running the commands. If you experience any issues during compilation, please open a ticket in our [GitHub repository](https://github.com/BugBlocker/lotus/issues) for assistance.

Alternatively, you can download the appropriate binary from the [GitHub Release page](https://github.com/BugBlocker/lotus/releases) and run it directly on your system.

To get started with example scripts, download the Lua scripts from our [GitHub Repository](https://github.com/BugBlocker/lotus-scripts). After downloading, you can run a test scan using the following command:

```bash
$ echo http://testphp.vulnweb.com/listproducts.php?cat=1 | lotus scan lotus-scripts/active -o test_out.json -v
```

## :book: Documentation

Detailed documentation is available in the `docs/*.md` folder of this repository. For a more accessible version, please visit our [online documentation](https://lotus.knas.me).
