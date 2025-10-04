<template>
  <v-sheet
    class="d-flex justify-between flex-column pa-3 rounded"
    style="height: 100%"
  >
    <div>
      <div class="text-h6 font-weight-bold mb-2">Inbox</div>
      <div v-if="inboxCount != 0">
        <div>Neue Scans:</div>
        <div class="text-center text-h1 my-4">{{ inboxCount }}</div>
      </div>
      <div v-else>
        <div class="text-center text-h5 my-4">Alles fertig</div>
      </div>
    </div>
    <div class="d-flex justify-start mt-auto">
      <v-btn to="/inbox" variant="elevated">Öffnen</v-btn>
      <v-btn
        variant="elevated"
        @click="inboxCount == 0 ? (inboxCount = 42) : (inboxCount = 0)"
        >Change Number</v-btn
      >
    </div>
  </v-sheet>
</template>

<script setup lang="ts">
import { ref } from "vue";

let inboxCount = ref(23); // unimplemented
</script>
