<template>
  <div class="mb-4">
    <div class="d-flex align-center justify-center ga-4">
      <v-text-field
        label="Search"
        prepend-inner-icon="mdi-magnify"
        hide-details
				variant="outlined"
      />
      <v-btn
        :icon="showFilters ? 'mdi-filter' : 'mdi-filter-outline'"
        variant="elevated"
        @click="toggleFilters"
      />
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
import { ref } from 'vue'

const showFilters = ref(false)
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

// Optional: Emit events für die Parent-Komponente
const emit = defineEmits(['filter-changed'])

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
