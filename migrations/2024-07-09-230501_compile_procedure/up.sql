CREATE PROCEDURE compile_items() LANGUAGE SQL AS 
$$ 

insert into
    p_s2t_item(
        item_name,
        item_uuid,
        category,
        tier,
        bid,
        sell_number
    )
SELECT
    s.item_name,
    MAX(s.item_uuid :: text) :: uuid AS item_uuid,
    MAX(s.category) AS category,
    MAX(s.tier) AS tier,
    AVG(s.starting_bid) AS starting_bid,
    COUNT(*) AS sell_number
FROM
    s2t_item s
GROUP BY
    s.item_name;

TRUNCATE s2t_item;

$$;