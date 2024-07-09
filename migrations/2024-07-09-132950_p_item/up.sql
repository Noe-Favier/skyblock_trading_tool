CREATE TABLE IF NOT EXISTS p_s2t_item (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    item_name TEXT NOT NULL,
    item_uuid UUID,
    category TEXT NOT NULL,
    tier TEXT NOT NULL,
    item_lore TEXT NOT NULL,
    starting_bid BIGINT NOT NULL,
    sell_number BIGINT NOT NULL
);