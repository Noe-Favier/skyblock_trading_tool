export interface Item {
    id: number;
    created_at: Date;
    item_name: string;
    item_uuid: string | null;
    category: string;
    tier: string;
    bid: bigint;
    sell_number: bigint;
    item_name_slug: string;
}