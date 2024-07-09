ALTER TABLE
    p_s2t_item
ADD
    COLUMN item_lore TEXT NOT NULL;

ALTER TABLE
    p_s2t_item RENAME COLUMN bid TO starting_bid;