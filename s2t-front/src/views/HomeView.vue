<script setup lang="ts">
import { ref, onMounted } from 'vue';
import ItemCardComponent from '../components/ItemCardView.vue';
import Loader from '../components/Loader.vue';
import { type Page } from '../models/page';

const page_result = ref<Page | null>(null);
const page_i = 0;
const error = ref<boolean>(false);
const loading = ref<boolean>(true); // État de chargement

onMounted(() => {
  fetch(`${import.meta.env.VITE_API_URL}/page/${page_i}`)
    .then(response => {
      if (!response.ok) {
        throw new Error('Erreur de réseau');
      }
      return response.json();
    })
    .then((data: Page) => {
      page_result.value = data;
    })
    .catch(() => {
      error.value = true;
    })
    .finally(() => {
      loading.value = false;
    });
});
</script>

<template>
  <main v-if="!loading && !error && page_result?.page">
    <template v-if="page_result.page.length === 0">
      <p>No items found</p>
    </template>
    <div v-for="item in page_result.page" :key="item.id">
      <ItemCardComponent class="item" :item="item" />
    </div>
  </main>
  <main class="stated-main" v-else>
    <div v-if="error">
      <div class="state-text error">Error fetching items</div>
    </div>
    <div v-else>
      <div class="state-tex">Loading</div>
      <Loader />
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

.stated-main {
  display: flex;
  justify-content: center;
  align-items: center;
}

.stated-main>div {
  text-align: center;
}

.item {
  width: 100%;
}

.error {
  color: red;
  font-weight: bold;
}

div.state-text {
  font-size: 1.5rem;
  margin-bottom: 1rem;
}
</style>
