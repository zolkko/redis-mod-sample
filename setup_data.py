import redis

r = redis.Redis(host='localhost', port=6379, db=0)
for i in range(100000 * 1000):
    r.set(f'test_{i}', i)
