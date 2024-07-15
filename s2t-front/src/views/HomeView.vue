<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ItemCardComponent from '../components/ItemCardView.vue';
import { type Page } from '../models/page';

const page_result = ref<Page | null>(null);
const page_i = 0;

onMounted(() => {
  fetch(`${import.meta.env.VITE_API_URL}/page/${page_i}`)
    .then(response => response.json())
    .then((data: Page) => {
      page_result.value = data;
    });
});
</script>

<template>
  <main v-if="page_result?.page">
    <div v-for="item in page_result.page" :key="item.id">
      <ItemCardComponent class="item" :item="item" />
    </div>
  </main>
</template>

<style scoped>
main {
  display: grid;
  width: 98%;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

.item {
  width: 100%;
}
</style>
