<template>
  <div class="home">
    <div class="wrapper">
      <div class="title">
        <p>Noten Verwaltung</p>
      </div>
      <Searchbar />
      <Dashboard />
    </div>
  </div>
</template>

<script setup lang="ts">
import Searchbar from '../components/Searchbar.vue'
import Dashboard from '../components/Dashboard.vue';
</script>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 12px;
}
.wrapper {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
  width: 100%;
  max-width: 50rem;
  height: max-content;
  /* padding: 1rem; */
}
.title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 1rem;
  font-family: 'DidactGothic', sans-serif;
  text-align: center;
}
</style>