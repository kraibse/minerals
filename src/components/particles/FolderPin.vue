<script setup lang="ts">
import { ref, defineProps, computed } from "vue";
import { invoke } from '@tauri-apps/api/tauri'

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'


const props = defineProps(["title", "path", "type"]);
const isOpen = ref(false);

const contents = ref([]);



const icon = computed(() => {
    if (props.type == "file") {
        return "fa-file";
    }
    else if (props.type == "folder") {
        return (isOpen.value) ? 'fa-folder-open' : 'fa-folder';
    }
    else {
        return "fa-question";
    }
});

async function getDirectoryContents() {
    try {
        await invoke('read_directory', { path: props.path }).then((result) => {
            console.log('Directory contents:', result);
            contents.value = result as [];
        });
    } catch (error) {
        console.error('Error reading directory:', error);
        return [];
    }
}

async function openDirectory() {
    if (props.type == "file") {
        return;
    }

    isOpen.value = !isOpen.value;
    console.log("openDirectory", isOpen.value);

    await getDirectoryContents();
}


</script>

<template>
    <li class="flex flex-col py-0.5 rounded-md cursor-pointer">
        <div class="flex flex-row items-center gap-x-2 hover:bg-[#2c313a]" @click="openDirectory">
            <FontAwesomeIcon :icon="(isOpen) ? 'fa-chevron-down' : 'fa-chevron-right'"
                class="text-gray-500 size-3 w-6" />
            <FontAwesomeIcon :icon="icon" class="text-blue-200" />
            <span class="text-gray-300 select-none line-clamp-1">{{ props.title }}</span>
        </div>

        <div class="flex flex-col pl-1 ml-3 border-l border-gray-500" v-if="isOpen && contents.length > 0">
            <div v-for="diritem, i in contents" :key="i">
                <FolderPin :title="diritem.name" :path="path + '\\' + diritem.name" :type="diritem.icon"></FolderPin>
            </div>
        </div>
    </li>
</template>

<style scoped></style>