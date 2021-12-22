import redis


r = redis.Redis(host='localhost', port=6379, db=0)

sum = 0
for key in r.scan_iter():
    if key.startswith(b"test_"):
        value = r.get(key)
        sum += int(value)
print(sum)
