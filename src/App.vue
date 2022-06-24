<script setup lang="ts">
</script>

<template>
  <p>It's just whatever, man</p>
  <p>
    <router-link to="/"> Home </router-link>
    <router-link to="/database-overview"> Database Overview </router-link>
  </p>
  <router-view></router-view>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
