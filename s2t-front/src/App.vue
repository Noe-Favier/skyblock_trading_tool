<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <nav>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="32px" height="32px" />
    <RouterLink to="/">Dashboard</RouterLink>
    <RouterLink to="/about">About</RouterLink>
  </nav>
  <div class="content">
    <RouterView />
  </div>
</template>

<style scoped>
template {
  width: 99vw;
  height: 100vh;
}

nav {
  text-align: center;
  display: flex;
  justify-content: flex-start;
  gap: 2vh;
  width: inherit;
  height: 4vh;
  padding-left: 3vw;
  align-items: center;
  border-radius: 16px;
  border-bottom: 1px solid #ccc;
}

.content {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 90vh;
  width: calc(100% - 16px);
  padding: 1vh;
}

RouterLink {
  background-color: aqua;
}
</style>
