<script setup>
// import { ref, defineProps } from "vue";

const props = defineProps(["title", "icon"]);
console.log("Found props", props);

</script>

<template>
    <li class="flex items-center space-x-2 py-1 rounded-md hover:bg-[#2c313a]">
        <FontAwesomeIcon :icon="['fa', 'fa-folder']"></FontAwesomeIcon>
        <span class="text-gray-300">{{ props.title }}</span>
    </li>
</template>

<style scoped></style>