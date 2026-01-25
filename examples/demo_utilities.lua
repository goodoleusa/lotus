-- Utility Classes Demo
-- This script demonstrates all new utility classes in Lotus v0.6.0

SCAN_TYPE = 1  -- Host only

function main()
    println("=== Lotus v0.6.0 Utility Classes Demo ===\n")

    -- ============================================
    -- Str Class - String Manipulation
    -- ============================================
    println("--- Str Class ---")

    local text = "  Hello, World!  "
    println("Original: '" .. text .. "'")
    println("Trim: '" .. Str:trim(text) .. "'")
    println("Lower: " .. Str:lower("HELLO"))
    println("Upper: " .. Str:upper("hello"))
    println("Contains 'World': " .. tostring(Str:contains(text, "World")))
    println("Starts with '  He': " .. tostring(Str:startswith(text, "  He")))
    println("Ends with '!  ': " .. tostring(Str:endswith(text, "!  ")))

    local parts = Str:split("a,b,c,d", ",")
    println("Split 'a,b,c,d': " .. parts[1] .. ", " .. parts[2] .. ", " .. parts[3])

    println("Replace: " .. Str:replace("hello", "l", "x"))
    println("Reverse: " .. Str:reverse("hello"))
    println("Length: " .. Str:len("hello"))
    println("Substring(2,4): " .. Str:sub("hello", 2, 4))
    println("Random(8): " .. Str:random(8))
    println("")

    -- ============================================
    -- Json Class - JSON Handling
    -- ============================================
    println("--- Json Class ---")

    local data = {
        name = "lotus",
        version = "0.6.0",
        features = {"Str", "Json", "Html", "Regex"}
    }

    local json_str = Json:encode(data)
    println("Encoded: " .. json_str)

    local decoded = Json:decode('{"id": 123, "active": true}')
    println("Decoded id: " .. decoded.id)
    println("Decoded active: " .. tostring(decoded.active))

    println("Pretty print:")
    println(Json:pretty({user = "admin", role = "superuser"}))
    println("")

    -- ============================================
    -- Html Class - HTML/CSS Selectors
    -- ============================================
    println("--- Html Class ---")

    local html = [[
        <html>
            <body>
                <h1 id="title">Welcome</h1>
                <a href="https://example.com" class="link">Click here</a>
                <p class="content">Hello <strong>World</strong></p>
            </body>
        </html>
    ]]

    local titles = Html:select(html, "h1#title")
    println("Select h1#title: " .. titles[1])

    local link = Html:select_one(html, "a.link")
    println("Select first link: " .. link)

    local href = Html:attr(link, "href")
    println("Link href: " .. href)

    local p = Html:select_one(html, "p.content")
    local text_content = Html:text(p)
    println("Text content: " .. text_content)

    -- XSS selector generation
    local xss_payload = '<img src=x onerror=alert(1)>'
    local selector = Html:xss_selector(xss_payload)
    println("XSS selector: " .. selector)

    -- HTML escape/unescape
    println("Escape: " .. Html:escape("<script>"))
    println("Unescape: " .. Html:unescape("&lt;script&gt;"))
    println("")

    -- ============================================
    -- Regex Class - Pattern Matching
    -- ============================================
    println("--- Regex Class ---")

    local test_str = "Contact: user@example.com or admin@test.org"

    println("Match email pattern: " .. tostring(Regex:match("\\w+@\\w+\\.\\w+", test_str)))

    local first_email = Regex:find("\\w+@\\w+\\.\\w+", test_str)
    println("First email: " .. first_email)

    local all_emails = Regex:find_all("\\w+@[\\w.]+", test_str)
    println("All emails: " .. all_emails[1] .. ", " .. all_emails[2])

    local replaced = Regex:replace("\\d+", "abc123def456", "X")
    println("Replace digits: " .. replaced)

    local split_result = Regex:split("\\s+", "hello   world  test")
    println("Split by whitespace: " .. split_result[1] .. ", " .. split_result[2] .. ", " .. split_result[3])

    local captures = Regex:captures("(\\w+)@(\\w+)", "user@domain")
    println("Captures: full=" .. captures[1] .. " user=" .. captures[2] .. " domain=" .. captures[3])
    println("")

    -- ============================================
    -- Global Utilities
    -- ============================================
    println("--- Global Utilities ---")
    println("randstr(12): " .. randstr(12))
    println("Sleeping for 0.5 seconds...")
    sleep(0.5)
    println("Done!")

    println("\n=== Demo Complete ===")
end
