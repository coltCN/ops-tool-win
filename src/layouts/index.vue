<template>
  <div class="main-container">
    <Header class="header" />
    <app-main class="main" />
  </div>
</template>
<script setup>
import { AppMain, Header } from './components'
</script>

<style lang="scss" scoped>
.main-container {
  display: flex;
  width: 100%;
  height: 100%;
  flex-direction: column;
  .header {
    height: 30px;
  }
  .main {
    flex: 1;
  }
}
</style>
