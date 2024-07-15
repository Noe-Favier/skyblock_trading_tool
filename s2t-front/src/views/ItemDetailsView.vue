<script lang="ts">
import { defineComponent, type PropType } from 'vue';
import type { Item } from '../models/item';
import type { ItemInfo } from '@/models/item-info';



</script>


<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';

let itemDetails = ref<ItemInfo | undefined>(undefined);
const route = useRoute();

fetch(`${import.meta.env.VITE_API_URL}/item/${route.params.name}`)
    .then(response => response.json())
    .then((data: ItemInfo) => {
        itemDetails.value = data;
    });

</script>

<template>
    <div v-if="itemDetails">
        <h1 class="title">{{ itemDetails.item.item_name }}</h1>
        <div>
            <p>actual bid : {{ itemDetails.item.bid }}</p>
            <p>category : {{ itemDetails.item.category }}</p>
            <p>tier : {{ itemDetails.item.tier }}</p>
        </div>

        <div>
            CHART GOES HERE
        </div>
    </div>
</template>

<style scoped>
.title {
    border-bottom: 1rem solid;
}
</style>