<script setup lang="ts">
import { defineProps } from "vue";

import SearchBar from "../atoms/SearchBar.vue";
import FolderGridItem from "../particles/FolderGridItem.vue";

const props = defineProps(["hasOptionBar"]);

</script>

<template>
    <div id="container-area" class="w-full h-full bg-red-500 flex flex-wrap align-items-stretch p-4 h-fit">
        <FolderGridItem v-for="i in 20" :key="i"></FolderGridItem>
    </div>
</template>

<style scoped>
#container-area {
    background-color: #24272e;
}
</style>