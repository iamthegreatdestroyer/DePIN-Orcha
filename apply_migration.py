import sqlite3
import sys

try:
    # Read the migration SQL
    with open('migrations/20260115000001_create_api_keys.sql', 'r') as f:
        sql = f.read()

    # Connect to the database
    conn = sqlite3.connect('depin_orcha.db')
    cursor = conn.cursor()

    # Execute the migration
    cursor.executescript(sql)
    conn.commit()

    print("✅ Migration applied successfully!")
    print("\nTables created:")
    cursor.execute("SELECT name FROM sqlite_master WHERE type='table'")
    for row in cursor.fetchall():
        print(f"  - {row[0]}")

    conn.close()
except Exception as e:
    print(f"❌ Error: {e}")
    sys.exit(1)
