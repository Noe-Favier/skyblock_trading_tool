<script setup lang="ts">
import ItemComponent from '../components/ItemView.vue'
let itemList: any = [];
let page = 0;

fetch(`${import.meta.env.VITE_API_URL}/page/${page}`)
  .then(response => response.json())
  .then(data => itemList = data.page);
</script>

<template>
  <main>
    <ul>
      <li v-for="item in itemList" :key="item.item_uuid">
        {{ item.item_name }}
      </li>
    </ul>
  </main>
</template>
