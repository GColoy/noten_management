<template>
	<v-sheet rounded class="pa-3 d-flex align-center justify-center flex-wrap ga-4" >
		<div @click="router.push({ path: '/Stück/' + piece.id })" class="text-center rounded cursor-pointer" style="width: 150px; height: 200px; background: yellowgreen;" v-for="piece in pieces" :key="piece.id">
			{{ piece.title }}
		</div>
	</v-sheet>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { getQuickAccessPieces } from '@/services/apiAccess'
const router = useRouter()
const pieces = getQuickAccessPieces()
</script>
