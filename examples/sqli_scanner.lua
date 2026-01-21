-- SQL Injection Scanner Example
-- Demonstrates: Str class, Regex class, sleep(), error handling

SCAN_TYPE = 2  -- Full URL with parameters

SQLI_PAYLOADS = {
    -- Error-based
    {payload = "'", errors = {"SQL syntax", "mysql_", "ORA-", "PostgreSQL"}},
    {payload = "\"", errors = {"SQL syntax", "mysql_", "ORA-", "PostgreSQL"}},
    {payload = "1'--", errors = {"SQL syntax", "mysql_"}},
    -- Boolean-based
    {payload = "' OR '1'='1", errors = {}},
    {payload = "' AND '1'='2", errors = {}},
}

function check_sql_errors(body, errors)
    for _, err in ipairs(errors) do
        if Str:contains(body, err) then
            return err
        end
    end
    return nil
end

function main()
    local params = HttpMessage:get_params()
    local base_url = HttpMessage:url()

    println("[*] Testing: " .. HttpMessage:host())
    println("[*] Parameters: " .. tostring(#params))

    -- Get baseline response
    local baseline_status, baseline = pcall(function()
        return http:send{url = base_url}
    end)

    if not baseline_status then
        println("[!] Cannot reach target")
        return
    end

    local baseline_length = Str:len(baseline.body)

    -- Test each parameter
    for param_name, original_value in pairs(params) do
        println("[*] Testing param: " .. param_name)

        for _, test in ipairs(SQLI_PAYLOADS) do
            -- Build payload
            local full_payload = original_value .. test.payload
            local test_url = HttpMessage:param_set(param_name, full_payload)

            -- Add small delay to avoid rate limiting
            sleep(0.1)

            local status, resp = pcall(function()
                return http:send{url = test_url}
            end)

            if status and resp then
                -- Check for SQL errors in response
                local found_error = check_sql_errors(resp.body, test.errors)

                if found_error then
                    Reports:add{
                        name = "SQL Injection (Error-based)",
                        risk = "critical",
                        url = resp.url,
                        param = param_name,
                        payload = test.payload,
                        evidence = found_error
                    }
                    println("[SQLi] Error-based in " .. param_name .. ": " .. found_error)
                end

                -- Check for significant response length difference (boolean-based)
                local resp_length = Str:len(resp.body)
                local diff = math.abs(resp_length - baseline_length)

                if diff > 100 and #test.errors == 0 then
                    -- Potential boolean-based SQLi
                    println("[?] Response length diff for " .. param_name .. ": " .. diff)
                end
            end
        end
    end

    println("[*] Scan complete")
end
