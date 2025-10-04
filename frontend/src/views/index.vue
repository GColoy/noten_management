<template>
  <div class="d-flex flex-column align-center ma-3">
    <div class="d-flex flex-column align-stretch justify-start w-100" style="max-width: 900px;">
      <div class="text-h3 pb-12 text-center" style="font-family: 'DidactGothic', sans-serif;">
        <p>Noten Verwaltung</p>
      </div>
      <Searchbar v-model="searchQuery" class="mb-8"/>
      <Dashboard />

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Searchbar from '../components/Searchbar.vue'
import Dashboard from '../components/Dashboard.vue';

// Search query for the searchbar component
const searchQuery = ref('')
</script>
