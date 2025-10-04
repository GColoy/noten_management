<template>
  <div class="mb-4">
    <div class="d-flex align-center justify-center ga-4">
      <v-text-field
        v-model="searchQuery"
        label="Search"
        prepend-inner-icon="mdi-magnify"
        hide-details
				variant="outlined"
        @keyup.enter="performSearch"
      />
      <v-btn
        :icon="showFilters ? 'mdi-filter' : 'mdi-filter-outline'"
        variant="elevated"
        @click="toggleFilters"
      />
      <v-btn
        icon="mdi-magnify"
        variant="elevated"
        @click="performSearch"
      />
      <!-- unimplemented -->
    </div>
    
    <v-expand-transition class="ps-0">
      <div v-if="showFilters" class="filter-row">
        <v-select
          v-model="selectedSubject"
          :items="subjects"
          label="Fach"
          variant="outlined"
          density="compact"
          clearable
          hide-details
					multiple
        />
        <v-select
          v-model="selectedGrade"
          :items="grades"
          label="Note"
          variant="outlined"
          density="compact"
          clearable
          hide-details
					multiple
        />
        <v-select
          v-model="selectedSemester"
          :items="semesters"
          label="Semester"
          variant="outlined"
          density="compact"
          clearable
          hide-details
					multiple
        />
        <v-select
          v-model="selectedYear"
          :items="years"
          label="Jahr"
          variant="outlined"
          density="compact"
          clearable
          hide-details
					multiple
        />
      </div>
    </v-expand-transition>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  disableNavigation: {
    type: Boolean,
    default: false
  }
})

const router = useRouter()

const showFilters = ref(false)

// Use computed property for two-way binding with parent
const searchQuery = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
})
const selectedSubject = ref(null)
const selectedGrade = ref(null)
const selectedSemester = ref(null)
const selectedYear = ref(null)

// Beispieldaten - diese sollten aus deiner API oder dem Store kommen
const subjects = ['Mathematik', 'Deutsch', 'Englisch', 'Geschichte', 'Biologie']
const grades = ['1', '2', '3', '4', '5', '6']
const semesters = ['1. Semester', '2. Semester']
const years = ['2023', '2024', '2025']

const toggleFilters = () => {
  showFilters.value = !showFilters.value
}

const performSearch = () => {
  const query = searchQuery.value.trim()
  if (query) {
    // Emit search event for parent to handle
    emit('search', query)
    
    // If navigation is not disabled, navigate to search page
    if (!props.disableNavigation) {
      router.push({
        name: 'search',
        query: { query }
      })
    }
  }
}

// Emit events for parent component
const emit = defineEmits(['update:modelValue', 'search', 'filter-changed'])

// Watchers für Filter-Änderungen
import { watch } from 'vue'
watch([selectedSubject, selectedGrade, selectedSemester, selectedYear], () => {
  emit('filter-changed', {
    subject: selectedSubject.value,
    grade: selectedGrade.value,
    semester: selectedSemester.value,
    year: selectedYear.value
  })
})
</script>

<style scoped>

.filter-row {
  display: flex;
  flex-direction: row;
  align-items: stretch;
	flex-wrap: wrap;
  gap: 1rem;
  margin-top: 1rem;
}

.filter-row > * {
	min-width: 150px;
}

@media (max-width: 768px) {
  .filter-row {
    flex-wrap: wrap;
		align-items: stretch;
    gap: 0.5rem;
  }
}
</style>
