<script setup lang="ts">

import FolderPin from "../particles/FolderPin.vue";
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import RoundedContainer from "../atoms/RoundedContainer.vue";
import SearchBar from "../atoms/SearchBar.vue";


</script>


<template>
    <div class="ring-slate-900/5 flex flex-row">

        <div class="h-16 bg-[#23272e] p-3 border-white flex flex-col justify-content-center gap-y-2">
            <FontAwesomeIcon icon="fa-folder" class="size-8 text-gray-600 cursors-pointer" v-for="i in 5" :key="i" />
        </div>
        <div id="folder-sidebar" class="ring-slate-900/5 bg-alternate flex flex-col overflow-y-auto scrollbar px-2 p-3">
            <h1 class="display-1 text-lg mb-2">
                <FontAwesomeIcon icon="fa-house" class="mr-2" />
                Development
            </h1>
            <SearchBar class="mb-2"></SearchBar>

            <div class="px-2">
                <FolderPin title="Home" icon="fa-home" class=""></FolderPin>

                <div id="pinned-folders" class="">
                    <FolderPin :title="'New Folder ' + i" v-for="i in 20" :key="i"></FolderPin>
                </div>
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