<template>
    <div id="rounded-container" class="p-1 rounded-lg bg-gray-600 flex justify-center items-center min-w-8 min-h-8">
        <slot></slot>
    </div>
</template>

<script setup lang="ts">

</script>

<style scoped>
#rounded-container {
    width: 100%;
    height: 100%;
}
</style>