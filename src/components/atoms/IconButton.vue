<script setup lang="ts">
import { ref, useSlots } from 'vue';

import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

const slots = useSlots();
const props = defineProps(["icon", "onClick"]);

const handleClick = () => {
    props.onClick();
}

</script>

<template>
    <div>
        <button @click="handleClick" class=" hover:border-active hover:bg-highlighted
            flex items-center text-white font-bold p-3 rounded-md m-1">
            <FontAwesomeIcon :icon="props.icon || 'fa-plus'" class="text-xl" />
        </button>
    </div>

</template>

<style scoped>
/* Your styles go here */
</style>
