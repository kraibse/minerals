<script setup>
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
            flex items-center text-white font-bold pl-2 pr-3 py-1 rounded-t-md rounded-tr-md m-1">
            <FontAwesomeIcon :icon="props.icon || 'fa-plus'" class="text-xl" :class="slots.content ? 'mr-2' : ''" />
            <p class="text-sm" v-if="slots.content">
                <slot></slot>
            </p>
        </button>
    </div>

</template>

<style scoped>
/* Your styles go here */
</style>
