-- API Fuzzer Example
-- Demonstrates: Json class, Regex class, Response helpers

SCAN_TYPE = 2  -- Full URL with parameters

function main()
    local base_url = HttpMessage:url()
    local host = HttpMessage:host()

    println("[*] Fuzzing API at: " .. host)

    -- Test for JSON endpoints
    local endpoints = {
        "/api/users",
        "/api/v1/users",
        "/api/config",
        "/api/debug",
        "/api/admin",
    }

    for _, endpoint in ipairs(endpoints) do
        local test_url = HttpMessage:set_path(endpoint)

        local status, resp = pcall(function()
            return http:send{url = test_url}
        end)

        if status then
            -- Check if response is JSON
            if resp:has_header("Content-Type") then
                local content_type = resp:get_header("Content-Type")

                if Str:contains(content_type, "application/json") then
                    -- Parse JSON response
                    local json_status, data = pcall(function()
                        return resp:json()
                    end)

                    if json_status and data then
                        println("[+] Found JSON endpoint: " .. endpoint)

                        -- Look for sensitive data patterns
                        local body = resp.body
                        local sensitive_patterns = {
                            {pattern = '"api_key"\\s*:', name = "API Key"},
                            {pattern = '"password"\\s*:', name = "Password"},
                            {pattern = '"token"\\s*:', name = "Token"},
                            {pattern = '"secret"\\s*:', name = "Secret"},
                        }

                        for _, p in ipairs(sensitive_patterns) do
                            if Regex:match(p.pattern, body) then
                                Reports:add{
                                    name = "Sensitive Data Exposure",
                                    risk = "high",
                                    url = test_url,
                                    finding = p.name .. " found in response",
                                    endpoint = endpoint
                                }
                                println("[!] " .. p.name .. " exposed at " .. endpoint)
                            end
                        end
                    end
                end
            end

            -- Check for interesting status codes
            if resp.status == 401 then
                println("[*] Auth required: " .. endpoint)
            elseif resp.status == 403 then
                println("[*] Forbidden (exists): " .. endpoint)
            elseif resp:status_ok() then
                println("[+] Accessible: " .. endpoint .. " (Status: " .. resp.status .. ")")
            end
        end
    end
end
