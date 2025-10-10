<template>
	<v-sheet class="flex-1-0 d-flex justify-between flex-column pa-3 rounded">
	      <div class="text-h6 font-weight-bold mb-2">Access Requests</div>
	      <v-list>
	        <v-list-item v-for="(request, index) in accessRequests" :key="index">
						<v-list-item-title>{{ request.Stück }}</v-list-item-title>
						<v-list-item-subtitle>{{ request.User }}</v-list-item-subtitle>
						<template v-slot:prepend>
							<v-icon v-if="request.Done" color="green">mdi-check-circle</v-icon>
							<v-icon v-else color="red">mdi-alert-circle</v-icon>
						</template>
	        </v-list-item>
	      </v-list>
	    </v-sheet>
</template>

<script setup lang="ts">
import { getAccessRequests } from '@/services/apiAccess'

let accessRequests = getAccessRequests()

</script>
