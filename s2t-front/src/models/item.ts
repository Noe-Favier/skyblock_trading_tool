export interface Item {
    id: number;
    created_at: { secs_since_epoch: number, nanos_since_epoch: number };
    item_name: string;
    item_uuid: string | null;
    category: string;
    tier: string;
    bid: number;
    sell_number: number;
    item_name_slug: string;
}