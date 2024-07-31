<script lang="ts">
import { defineComponent, type PropType } from 'vue';
import type { Item } from '../models/item';

export default defineComponent({
    name: 'ItemCard',
    props: {
        item: {
            type: Object as PropType<Item>,
            required: true
        }
    }
});
</script>

<template>
    <div class="card" v-on:click="() => $router.push({ name: 'item', params: { name: item.item_name_slug } })">
        <div :class="['card-header', `rarity-${item.tier.toLowerCase()}`]">
            <div class="tier">{{ item.tier }}</div>
            <h3 class="card-title">{{ item.item_name }}</h3>
        </div>
        <div class="card-body">
            <p class="category">{{ item.category }}</p>
            <p class="bid">{{ item.bid }}</p>
        </div>
    </div>
</template>

<style scoped>
.card {
    display: flex;
    flex-direction: column;
    border-radius: 10px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    transition: transform 0.2s;
    margin: 1rem;
    cursor: pointer;
}

.card:hover {
    transform: translateY(-10px);
}

.card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem;
    background-color: #f8f9fa;
    border-bottom: 1px solid #e9ecef;
}

.tier {
    font-size: 0.85rem;
    font-weight: bold;
    color: #fff;
    padding: 0.25rem 0.5rem;
    border-radius: 5px;
    text-transform: uppercase;
    background-color: #6c757d;
    /* Default background for tier */
}

.card-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: bold;
    color: #343a40;
}

.card-body {
    padding: 1rem;
}

.category,
.bid {
    margin: 0;
    font-size: 1rem;
    color: #6c757d;
}
</style>
