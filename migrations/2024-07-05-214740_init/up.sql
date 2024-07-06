CREATE TABLE IF NOT EXISTS s2t_item (
    auction_id UUID PRIMARY KEY,
    item_name TEXT NOT NULL,
    item_uuid UUID,
    category TEXT NOT NULL,
    tier TEXT NOT NULL,
    item_lore TEXT NOT NULL,
    starting_bid BIGINT NOT NULL
);