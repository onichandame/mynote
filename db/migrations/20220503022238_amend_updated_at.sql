-- Add migration script here
update notes set updated_at = coalesce(updated_at, deleted_at, created_at) where updated_at is null;