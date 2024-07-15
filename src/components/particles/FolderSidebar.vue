<script setup lang="ts">
import FolderPin from "./FolderPin.vue";
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import RoundedContainer from "../atoms/RoundedContainer.vue";
import SearchBar from "../atoms/SearchBar.vue";

</script>


<template>
    <div class="flex flex-row ring-slate-900/5 p-2">

        <div>
            <div class="w-full h-16 dark:bg-slate-800 border-1 border-bottom border-white flex flex-col gap-y-2">
                <RoundedContainer class="flex justify-center items-center size-12 cursor-pointer" v-for="i in 6"
                    :key="i">
                    <FontAwesomeIcon icon="fa-folder" />
                </RoundedContainer>
            </div>
        </div>
        <div id="folder-sidebar" class="ring-slate-900/5 flex flex-col overflow-y-auto scrollbar px-2">
            <h1 class="display-1 text-xl">DESKTOP-QUANTUM1</h1>
            <SearchBar></SearchBar>

            <FolderPin title="Home" icon="fa-home" class="mt-2"></FolderPin>

            <div id="pinned-folders" class="m-0">
                <FolderPin :title="'New Folder ' + i" v-for="i in 20" :key="i"></FolderPin>
            </div>

            <hr class="my-4">

            <div id="drives"></div>

        </div>
    </div>

</template>

<style scoped>
#folder-sidebar {
    max-width: 400px;
    width: 320px;
    height: 100vh;
}
</style>