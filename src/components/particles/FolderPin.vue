<script setup lang="ts">
import { ref, defineProps } from "vue";
import { invoke } from '@tauri-apps/api/tauri'

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'


const props = defineProps(["title", "path", "type"]);
const isOpen = ref(false);


class DirElement {
    title: string;
    type: 'folder' | 'file';
    path: string;

    contents: DirElement[];

    constructor(title: string, path: string, type: 'folder' | 'file') {
        this.path = path;

        this.title = title;

        this.type = type;
        this.contents = [];
    }

    async getDirectoryContents() {
        try {
            const contents = await invoke('read_directory', { path: this.path });
            console.log('Directory contents:', contents);
            return contents;
        } catch (error) {
            console.error('Error reading directory:', error);
            return [];
        }
    }
}

const thisDirectory: DirElement = new DirElement(props.title, props.path, props.type);

const contents = thisDirectory.getDirectoryContents();

</script>

<template>
    <li class="flex flex-col py-0.5 rounded-md cursor-pointer">
        <div class="flex flex-row items-center gap-x-2 hover:bg-[#2c313a]" @click="isOpen = !isOpen">
            <FontAwesomeIcon :icon="(isOpen) ? 'fa-chevron-down' : 'fa-chevron-right'"
                class="text-gray-500 size-3 w-6" />
            <FontAwesomeIcon icon="fa-folder" class="text-blue-200" />
            <span class="text-gray-300 select-none">{{ props.title }}</span>
        </div>

        <div class="flex flex-col pl-1 ml-3 border-l border-gray-500" v-if="isOpen && contents.length > 0">
            <div v-for="diritem, i in thisDirectory.contents" :key="i">
                <FolderPin :title="diritem.title" :icon="diritem.type"></FolderPin>
            </div>
        </div>
    </li>
</template>

<style scoped></style>