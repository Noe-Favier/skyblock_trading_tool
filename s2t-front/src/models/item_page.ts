import type { Item } from './item';

export interface Page {
    page_size: number;
    total_pages: number;
    total_items: number;
    items_in_page: number;
    page: Item[];
}