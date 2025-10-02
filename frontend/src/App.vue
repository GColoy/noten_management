<template>
  <v-layout>
    <v-fab app location="top right" icon>
      <v-icon>mdi-menu</v-icon>
      <v-menu activator="parent">
        <v-list density="compact" nav>
          <v-list-item 
            :prepend-icon="theme.current.value.dark ? 'mdi-weather-sunny' : 'mdi-weather-night'" 
            link 
            :title="theme.current.value.dark ? 'Light Theme' : 'Dark Theme'" 
            @click="theme.toggle()" />
          <v-list-item prepend-icon="mdi-cog-outline" link title="Settings" />
          <v-list-item prepend-icon="mdi-logout" link title="Logout" />
        </v-list>
      </v-menu>
    </v-fab>
    <v-main>
      <router-view max-width="100%"/>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { useTheme } from 'vuetify'

  const theme = useTheme()

  const drawer = ref(true)

  const items = ref([
    {
      title: 'Dashboard',
      prependIcon: 'mdi-view-dashboard-outline',
      link: true,
      to: '/dashboard',
    },
    {
      title: 'Profile',
      prependIcon: 'mdi-account-outline',
      link: true,
      to: '/profile',
    }
  ])
</script>

<style>
@font-face {
    font-family: 'DidactGothic';
    src: url('@/assets/fonts/DidactGothic.ttf') format('truetype');
}
</style>