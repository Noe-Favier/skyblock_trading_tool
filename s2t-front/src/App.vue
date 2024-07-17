<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <nav>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="32px" height="32px" />
    <div class="btn">
      <RouterLink to="/">Dashboard</RouterLink>
      <RouterLink to="/about">About</RouterLink>
    </div>
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
  display: flex;
  justify-content: flex-start;
  width: inherit;
  height: 4vh;
  gap: 2vh;
  padding-left: 1vw;
  align-items: center;
  border-radius: 16px;
  border-bottom: 1px solid #ccc;
}

.btn {
  display: flex;
  justify-content: space-between;
  flex-grow: 1;
  padding-right: 1vw;

}

.content {
  display: flex;
  height: 90vh;
  width: calc(100% - 16px);
  padding: 1vh;
  overflow-y: auto;
}

RouterLink {
  background-color: aqua;
}
</style>
