-- Chainable API Demo
-- Lotus v0.6.0 - Fluent Interface Examples

SCAN_TYPE = 1  -- Host only

function main()
    println("=== Lotus v0.6.0 Chainable API Demo ===\n")

    -- ============================================
    -- str() - String Operations with Chaining
    -- ============================================
    println("--- String Chaining ---")

    -- Basic transformations
    local result = str("  Hello World  "):trim():upper():value()
    println("trim + upper: " .. result)  -- "HELLO WORLD"

    -- Multiple operations chained
    str("hello"):upper():append("!"):prepend("["):append("]"):print()
    -- "[HELLO!]"

    -- Split and process
    str("apple,banana,cherry")
        :split(",")
        :map(function(x) return str(x):upper():value() end)
        :join(" | ")
        :print()
    -- "APPLE | BANANA | CHERRY"

    -- Regex with chaining
    local emails = str("Contact: admin@test.com or user@example.org")
        :find_all("[\\w]+@[\\w.]+")
        :value()
    println("Found emails: " .. #emails)

    -- Encoding chain
    str("<script>alert('xss')</script>")
        :html_encode()
        :print()

    str("hello world"):base64_encode():print()
    str("aGVsbG8gd29ybGQ="):base64_decode():print()

    println("")

    -- ============================================
    -- html() - HTML/CSS Selector Chaining
    -- ============================================
    println("--- HTML Chaining ---")

    local page = [[
        <html>
            <body>
                <div class="users">
                    <a href="/user/1" class="user">Alice</a>
                    <a href="/user/2" class="user">Bob</a>
                    <a href="/user/3" class="user admin">Charlie</a>
                </div>
            </body>
        </html>
    ]]

    -- Select and get attributes
    html(page)
        :select("a.user")
        :each(function(el)
            local name = el:text():value()
            local href = el:attr("href"):value()
            println("User: " .. name .. " -> " .. href)
        end)

    -- Get first admin
    local admin = html(page):select_one("a.admin")
    if admin then
        println("Admin found: " .. admin:text():value())
    end

    -- Count elements
    local user_count = html(page):select("a.user"):len()
    println("Total users: " .. user_count)

    println("")

    -- ============================================
    -- json() - JSON Navigation Chaining
    -- ============================================
    println("--- JSON Chaining ---")

    local api_response = [[
        {
            "status": "success",
            "data": {
                "users": [
                    {"id": 1, "name": "Alice", "role": "admin"},
                    {"id": 2, "name": "Bob", "role": "user"},
                    {"id": 3, "name": "Charlie", "role": "user"}
                ],
                "total": 3
            }
        }
    ]]

    -- Navigate with dot notation
    local status = json(api_response):get("status"):value()
    println("Status: " .. status)

    -- Get nested array element
    local first_user = json(api_response):get("data.users.0.name"):value()
    println("First user: " .. first_user)

    -- Iterate users
    json(api_response):get("data.users"):each(function(user)
        local name = user:get("name"):value()
        local role = user:get("role"):value()
        println("  - " .. name .. " (" .. role .. ")")
    end)

    -- Get keys
    json(api_response):get("data"):keys():print()

    println("")

    -- ============================================
    -- tbl() - Table Operations
    -- ============================================
    println("--- Table Chaining ---")

    local payloads = tbl({"<script>", "<img src=x>", "normal", "<svg onload>"})

    -- Filter XSS payloads
    payloads
        :filter(function(p) return str(p):contains("<") end)
        :each(function(p) println("XSS payload: " .. p) end)

    -- Sort and unique
    tbl({"z", "a", "m", "a", "z"})
        :unique()
        :sort()
        :join(", ")
        :print()
    -- "a, m, z"

    println("")

    -- ============================================
    -- Practical Security Example
    -- ============================================
    println("--- Practical Example ---")

    local xss_payload = "<img src=x onerror=alert(1)>"

    -- Generate XSS detection selector
    local selector = html(xss_payload):xss_selector():value()
    println("XSS Selector: " .. selector)

    -- Encode for safe display
    local safe = str(xss_payload):html_encode():value()
    println("Safe display: " .. safe)

    -- Generate random token
    local token = randstr(32):value()
    println("CSRF Token: " .. token)

    println("\n=== Demo Complete ===")
end
