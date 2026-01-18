-- Add unique constraint for rate limit deduplication
-- This allows ON CONFLICT to work properly in INSERT statements

CREATE UNIQUE INDEX IF NOT EXISTS idx_rate_limit_unique
ON rate_limit_log(api_key_id, endpoint, window_start);
