<script setup lang="ts">
import FolderPin from "./FolderPin.vue";

</script>


<template>
    <div id="folder-sidebar" class="p-3 border-0 border-gray-300 flex flex-col">
        <h1 class="display-1 text-2xl">DESKTOP-QUANTUM1</h1>

        <FolderPin title="Home" icon="fa-home" class="mt-2"></FolderPin>

        <hr class="my-4">

        <div id="pinned-folders" class="m-0">
            <FolderPin :title="'New Folder ' + i" v-for="i in 10" :key="i"></FolderPin>
        </div>

        <hr class="my-4">

        <div id="drives"></div>

    </div>
</template>

<style scoped>
#folder-sidebar {
    max-width: 400px;
    width: 320px;
    height: 100vh;
}
</style>