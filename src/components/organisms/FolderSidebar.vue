<script setup lang="ts">
import { ref } from 'vue';

import FolderPin from "../particles/FolderPin.vue";
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import RoundedContainer from "../atoms/RoundedContainer.vue";
import SearchBar from "../atoms/SearchBar.vue";


const activeTab = ref(1);
const drives = ref({
    "OS": "C:",
    "Games": "D:",
    "Music": "E:",
    "Pictures": "F:",
});

</script>


<template>
    <div class="ring-slate-900/5 flex flex-row">

        <div class="h-16 bg-[#23272e] border-white flex flex-col justify-content-center gap-y-2">
            <FontAwesomeIcon :icon="(i == 1) ? 'fa-home' : 'fa-folder'"
                class="size-6 px-3 pb-1.5 text-gray-600 hover:text-blue-200 cursor-pointer rounded-l-xl" v-for="i in 5"
                :class="{ 'pt-3': i === 1, 'pb-3': i === 5, 'active': activeTab === i }" :key="i" />
        </div>
        <div id="folder-sidebar"
            class="ring-slate-900/5 border-l-4 border-active bg-alternate flex flex-col overflow-hidden px-2 pb-3">

            <h1 class="display-1 text-lg mb-2 px-3 py-2 h-fit">
                <!-- <FontAwesomeIcon icon="fa-house" class="mr-2" /> -->
                Development
            </h1>
            <SearchBar class="mb-2"></SearchBar>

            <div class="flex flex-col scrollbar overflow-y-auto">
                <div class="px-2">
                    <FolderPin title="Home" icon="fa-home" class=""></FolderPin>

                    <div id="pinned-folders" class="">
                        <FolderPin :title="'New Folder ' + i" v-for="i in 20" :key="i"></FolderPin>
                    </div>
                </div>


                <hr class="my-4">

                <div id="drives">
                    <FolderPin v-for="drive in Object.keys(drives)" :key="drive" :title="drive" :path="drives[drive]"
                        type="folder"></FolderPin>
                </div>
            </div>

        </div>
    </div>

</template>

<style scoped>
#folder-sidebar {
    max-width: 400px;
    width: 320px;
    height: 100vh;
}

.active {
    background-color: var(--primary-color);
    color: white;
    padding-bottom: 0.75rem;
}
</style>