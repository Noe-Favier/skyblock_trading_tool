<script lang="ts">
import { defineComponent, type PropType } from 'vue';
import type { Item } from '../models/item';
import type { ItemInfo } from '@/models/item-info';



</script>


<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import { LineChart } from 'vue-chart-3';

let itemDetails = ref<ItemInfo | undefined>(undefined);
const route = useRoute();

fetch(`${import.meta.env.VITE_API_URL}/item/${route.params.name}`)
    .then(response => response.json())
    .then((data: ItemInfo) => {
        itemDetails.value = data;
    });

</script>

<template>
    <div v-if="itemDetails" class="master">
        <h1 class="title" :class="['decoration-rarity-' + itemDetails.item.tier.toLowerCase()]">
            {{ itemDetails.item.item_name }}
        </h1>
        <div>
            <p>actual bid : {{ itemDetails.item.bid }}</p>
            <p>category : {{ itemDetails.item.category }}</p>
            <p>tier : {{ itemDetails.item.tier }}</p>
        </div>

        <div>
            <!--
            <LineChart .... />
            -->
        </div>
    </div>
</template>

<style scoped>
.title {
    width: fit-content;
    text-decoration: underline;
}

.master {
    background-color: #FFF !important;
    padding: 0 1%;
    width: 100%;
    border-radius: 12px;
}
</style>