<script setup>
import SearchBar from "../atoms/SearchBar.vue";
import TextButton from "../atoms/TextButton.vue";
import IconButton from "../atoms/IconButton.vue";

</script>

<template>

    <div class="w-full flex justify-between items-center bg-alternate">
        <div id="searchbar" class="h-8 bg-alternate flex flex-row items-center justify-start py-2">
            <!-- <SearchBar class="" /> -->
            <IconButton :icon="'fa-arrow-left'"></IconButton>
            <IconButton :icon="'fa-arrow-right'"></IconButton>
            <IconButton :icon="'fa-arrow-up'"></IconButton>
        </div>

        <div class="flex w-full my-1 mx-2">
            <input type="text" name="" value=""
                class="w-full px-2 py-0.5 focus:outline-none focus:ring-blue-500 focus:ring-2"
                placeholder="C:\Windows\System32">
        </div>

        <div id="tools" class="h-8 bg-alternate flex flex-row items-center py-2">
            <TextButton :icon="'fa-folder'"></TextButton>
            <TextButton :icon="'fa-folder-plus'"></TextButton>
            <TextButton :icon="'fa-folder-open'"></TextButton>
            <TextButton :icon="'fa-file'"></TextButton>
            <TextButton :icon="'fa-trash'"></TextButton>
        </div>
    </div>

</template>

<style scoped>
input[type=text] {
    background-color: #0f0f0f98;
    border: none;
    border-radius: 0.5rem;
    color: #ffffff;
}
</style>
