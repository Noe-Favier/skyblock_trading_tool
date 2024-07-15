import type { Item } from "./item"

export interface ItemInfo {
    item: Item,
    versions: Item[],
    image_url: string,
}