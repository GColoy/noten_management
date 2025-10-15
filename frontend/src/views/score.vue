<template>
<DefaultPage>
<h1 class="text-center">{{ Route.params.id }}</h1>  
</DefaultPage>
</template>

<script setup lang="ts">
import DefaultPage from "@/components/DefaultPage.vue";
import { useRoute } from "vue-router";
import { getScoreInfos } from "@/services/apiAccess.ts"
const Route = useRoute();
const Score = getScoreInfos();
</script>