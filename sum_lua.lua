local cursor = "0";
local sum = 0;

repeat
    local result = redis.call("SCAN", cursor, "MATCH", "test_*");
    cursor = result[1];
    local t = result[2];
    for i, key in pairs(t) do
        local value = redis.call("GET",key);
        sum = sum + tonumber(value);
    end;
until cursor == "0";
return sum;

