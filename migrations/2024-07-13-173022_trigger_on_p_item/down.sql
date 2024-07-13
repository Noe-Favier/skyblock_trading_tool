ALTER TABLE p_s2t_item DROP COLUMN item_name_slug;

DROP TRIGGER IF EXISTS set_item_name_slug ON p_s2t_item;