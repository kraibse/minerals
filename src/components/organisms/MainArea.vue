<script setup>
import { ref } from 'vue';
import ContainerArea from './ContainerArea.vue';
import SearchBar from '../atoms/SearchBar.vue';
import ToolTopBar from './ToolTopBar.vue';

</script>

<template>
    <div class="relative flex flex-col w-full h-full">
        <ToolTopBar></ToolTopBar>

        <ContainerArea></ContainerArea>
    </div>

</template>

<style scoped>
/* Your styles go here */
</style>
