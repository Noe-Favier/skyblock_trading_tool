ALTER TABLE
    p_s2t_item ADD COLUMN item_name_slug TEXT NOT NULL DEFAULT 'INVALID';

CREATE OR REPLACE FUNCTION update_item_name_slug()
RETURNS TRIGGER AS $$
BEGIN
  NEW.item_name_slug := slugify(NEW.item_name);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_item_name_slug
BEFORE INSERT OR UPDATE ON p_s2t_item
FOR EACH ROW
EXECUTE FUNCTION update_item_name_slug();

UPDATE p_s2t_item SET item_name_slug = slugify(item_name);