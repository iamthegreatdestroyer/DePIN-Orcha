#!/usr/bin/env python3
"""
Bootstrap script to create an admin API key for DePIN-Orcha
"""
import sqlite3
import hashlib
import base64
import uuid
from datetime import datetime
import json
import sys

def create_admin_key(db_path="depin_orcha.db", name="Admin Bootstrap Key", rate_limit=1000):
    """Create an admin API key and insert into database"""

    # Generate API key
    key_uuid = uuid.uuid4().hex
    api_key = f"dpn_{key_uuid}"

    # Hash the key using bcrypt (same as middleware expects)
    import bcrypt
    key_bytes = api_key.encode('utf-8')
    key_hash = bcrypt.hashpw(key_bytes, bcrypt.gensalt()).decode('utf-8')

    # Current timestamp in ISO format
    now = datetime.utcnow().strftime('%Y-%m-%dT%H:%M:%S.%f')

    # Permissions
    permissions = json.dumps(["read", "write", "admin", "delete"])

    # Connect to database
    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()

        # Insert the key
        cursor.execute("""
            INSERT INTO api_keys
                (key_hash, name, description, created_at, is_active, rate_limit_per_minute, permissions)
            VALUES
                (?, ?, ?, ?, 1, ?, ?)
        """, (key_hash, name, "Bootstrap admin key", now, rate_limit, permissions))

        conn.commit()
        key_id = cursor.lastrowid
        conn.close()

        print("=" * 60)
        print("✅ Admin API Key Created Successfully!")
        print("=" * 60)
        print(f"Key ID: {key_id}")
        print(f"API Key: {api_key}")
        print(f"Name: {name}")
        print(f"Rate Limit: {rate_limit} requests/minute")
        print(f"Permissions: {', '.join(json.loads(permissions))}")
        print()
        print("Usage: Add this header to your API requests:")
        print(f"  X-API-Key: {api_key}")
        print()
        print("⚠️  IMPORTANT: Save this key securely - it cannot be retrieved later!")
        print("=" * 60)

        return api_key

    except sqlite3.Error as e:
        print(f"❌ Database error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"❌ Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='Create admin API key for DePIN-Orcha')
    parser.add_argument('--db', default='depin_orcha.db', help='Database file path')
    parser.add_argument('--name', default='Admin Bootstrap Key', help='Key name')
    parser.add_argument('--rate-limit', type=int, default=1000, help='Rate limit per minute')

    args = parser.parse_args()

    create_admin_key(args.db, args.name, args.rate_limit)
