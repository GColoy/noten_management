<template>
  <div class="d-flex align-stretch flex-wrap w-100 ga-4">
    <QuickSelect class="flex-1-1-100" :style="'min-height: ' + height"/>
    <InboxDashboard class="flex-1-0" :style="'min-height: ' + height + ';' + 'min-width: ' + height"/>
    <AccessReuestDashboard class="flex-1-0" :style="'min-height: ' + height + ';' + 'min-width: ' + height"/>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ref } from 'vue'
import QuickSelect from './QuickSelect.vue'
import InboxDashboard from './InboxDashboard.vue'
import AccessReuestDashboard from './AccessReuestDashboard.vue'
const height = '250px'
const router = useRouter()

</script>