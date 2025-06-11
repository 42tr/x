<script setup lang="ts">
import ApplicantList from './components/ApplicantList.vue'
</script>

<template>
  <div class="header">
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
    <h1>Resume Assessment Tool</h1>
  </div>
  <main>
    <ApplicantList />
  </main>
</template>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 2rem;
  gap: 1rem;
}

.logo {
  height: 4em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}

main {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
}
</style>
