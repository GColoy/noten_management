<template>
  <div class="search-page">
    <h2 class="mb-4">Search</h2>
    
    <!-- Include searchbar component for editing query -->
    <Searchbar 
      v-model="query"
      :disable-navigation="true"
      @search="handleSearch"
      class="mb-6"
      
    />
    
    <div v-if="!query" class="text-center py-8">
      <v-icon size="64" color="grey-lighten-1">mdi-magnify</v-icon>
      <p class="text-grey mt-4">Enter a search query above</p>
    </div>
    
    <div v-else-if="loading" class="text-center py-8">
      <v-progress-circular indeterminate color="primary" size="64" />
      <p class="mt-4">Searching...</p>
    </div>
    
    <div v-else-if="results.length === 0" class="text-center py-8">
      <v-icon size="64" color="grey-lighten-1">mdi-file-search-outline</v-icon>
      <p class="text-grey mt-4">No results found for "{{ query }}"</p>
    </div>
    
    <div v-else>
      <p class="mb-4 text-grey">Found {{ results.length }} results for "{{ query }}"</p>
      
      <v-card v-for="result in results" :key="result.id" class="mb-4">
        <v-card-text>
          <h3>{{ result.title }}</h3>
          <p class="text-grey">{{ result.description }}</p>
        </v-card-text>
      </v-card>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import Searchbar from '@/components/Searchbar.vue'

const route = useRoute()
const router = useRouter()

const query = ref('')
const results = ref([])
const loading = ref(false)

// Handle search from searchbar component
const handleSearch = (searchQuery) => {
  query.value = searchQuery
  // Update URL without navigation
  router.replace({
    name: 'search',
    query: { query: searchQuery }
  })
  performSearch()
}

// Get query from URL parameters
const updateQuery = () => {
  const urlQuery = route.query.query || ''
  query.value = urlQuery
  if (urlQuery) {
    performSearch()
  }
}

// Mock search function - replace with actual search logic
const performSearch = async () => {
  loading.value = true
  
  try {
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // Mock results - replace with actual search results
    results.value = [
      {
        id: 1,
        title: `Sample result for "${query.value}"`,
        description: 'This is a mock search result. Replace with actual search functionality.'
      },
      {
        id: 2,
        title: `Another result for "${query.value}"`,
        description: 'Another mock search result to demonstrate the layout.'
      }
    ]
  } catch (error) {
    console.error('Search error:', error)
    results.value = []
  } finally {
    loading.value = false
  }
}

// Watch for route changes
watch(() => route.query.query, updateQuery)

onMounted(() => {
  updateQuery()
})
</script>

<style scoped>
.search-page {
  max-width: 800px;
  margin: 0 auto;
  padding: 1rem;
}
</style>