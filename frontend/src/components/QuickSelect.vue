<template>
	<v-sheet rounded class="pa-3 d-flex align-center justify-center flex-wrap ga-4" >
	      <div @click="router.push('/unimplemented')" class="text-center rounded cursor-pointer" style="width: 150px; height: 200px; background: yellowgreen;" v-for="value in [1,2,3]" :key="value">
	        Stück
	      </div>
	    </v-sheet>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>
