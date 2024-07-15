<script setup lang="ts">
import TheWelcome from '../components/TheWelcome.vue'
let itemList = [];
fetch('https://testapi.jasonwatmore.com/products/1')
  .then(response => response.json())
  .then(data => itemList = data);
</script>

<template>
  <main>
    <TheWelcome />
  </main>
</template>
