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
    justify-content: space-between;
    border-radius: 10px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    transition: transform 0.2s;
    margin: 1rem;
}

.tier {
    transform: rotate(-90deg);
    font: 8px sans-serif;
    margin: 0;
    padding: 0;
    text-align: center;
    width: 50px;
    height: 23px;
}

.card:hover {
    transform: translateY(-10px);
    cursor: pointer;
}

.card-header {
    display: flex;
    padding: 1rem;
    padding-left: 0;
    border-bottom: 1px solid #e9ecef;
    text-align: center;
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

.card-description {
    margin: 0;
    font-size: 1rem;
    color: #6c757d;
}
</style>