import sqlite3

conn = sqlite3.connect('depin_orcha.db')
cursor = conn.cursor()

print("📊 Database Tables:")
cursor.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
for row in cursor.fetchall():
    print(f"  - {row[0]}")

print("\n🔑 API Keys:")
cursor.execute("SELECT COUNT(*) FROM api_keys")
print(f"  Count: {cursor.fetchone()[0]}")

print("\n📈 Rate Limit Log:")
cursor.execute("SELECT COUNT(*) FROM rate_limit_log")
print(f"  Count: {cursor.fetchone()[0]}")

conn.close()
