<template>
  <div class="searchbar">
	<v-text-field
		label="Search"
		prepend-inner-icon="mdi-magnify"
		hide-details
	/>
	<v-btn
		icon="mdi-filter-outline"
		type="elevated"
	/>
  </div>
</template>

<style scoped>
.searchbar {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  margin-bottom: 1rem;
}
</style>
