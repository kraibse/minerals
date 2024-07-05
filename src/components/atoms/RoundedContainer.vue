<template>
    <div id="rounded-container" class="p-1 rounded-lg bg-gray-600 flex align-items-center">
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