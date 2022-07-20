-- Wrap migration in transaction to make sure
-- it succeeds or fails automatically.  `sqlx` does not automatically
-- do transactions for us.
BEGIN;
    -- Backfill 'status' for historical entries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make `status` mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
