<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import FolderSidebar from "./components/organisms/FolderSidebar.vue";
import MainArea from "./components/organisms/MainArea.vue";
import RightSidebar from "./components/organisms/RightSidebar.vue";

import IconButton from "./components/atoms/IconButton.vue";


</script>

<template>
  <div class="flex flex-col">
    <!-- <div class="h-fit flex flex-row">
      <IconButton></IconButton>
      <IconButton>Tab 1</IconButton>
      <IconButton>New File</IconButton>
      <IconButton>New File</IconButton>
      <IconButton>New File</IconButton>
    </div> -->

    <div class="flex flex-row">
      <FolderSidebar />
      <MainArea />
      <!-- <RightSidebar /> -->
    </div>
  </div>

</template>

<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #f6f6f6;
  background-color: #23272e;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0 !important;
  padding: 0;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
