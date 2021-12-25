local result = redis.call("redsum", "test_");
local sum = tonumber(result);
return sum;

