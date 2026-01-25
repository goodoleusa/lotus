-- XSS Scanner Example
-- Demonstrates: Html class, Str class, HttpMessage methods, Response helpers

SCAN_TYPE = 2  -- Full URL with parameters

XSS_PAYLOADS = {
    '<script>alert(1)</script>',
    '<img src=x onerror=alert(1)>',
    '<svg onload=alert(1)>',
    '"><script>alert(1)</script>',
}

function main()
    local base_url = HttpMessage:url()
    local params = HttpMessage:get_params()

    -- Test each parameter
    for param_name, _ in pairs(params) do
        for _, payload in ipairs(XSS_PAYLOADS) do
            -- Create test URL
            local test_url = HttpMessage:param_set(param_name, payload)

            -- Send request
            local status, resp = pcall(function()
                return http:send{url = test_url}
            end)

            if status and resp:status_ok() then
                -- Generate CSS selector for the payload
                local selector = Html:xss_selector(payload)

                -- Check if payload is reflected
                if selector ~= "" then
                    local matches = Html:select(resp.body, selector)
                    if #matches > 0 then
                        -- Found XSS!
                        Reports:add{
                            name = "Reflected XSS",
                            risk = "high",
                            url = resp.url,
                            param = param_name,
                            payload = payload,
                            evidence = matches[1]
                        }
                        println("[XSS] Found in param: " .. param_name)
                    end
                end
            end
        end
    end
end
