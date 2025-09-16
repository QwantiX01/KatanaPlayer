<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, watch } from "vue";

const volumePercentage = ref(100);

async function pause() {
    await invoke("pause");
}

async function play() {
    await invoke("play");
}

async function setVolume(perc: Number) {
    await invoke("set_volume", { percentage: perc });
}

watch(volumePercentage, async (newValue) => {
    console.log(newValue);
    await setVolume(Number(newValue));
});
</script>

<template>
    <main class="">
        <button @click="play">PLay</button>
        <button @click="pause">Pause</button>
        <input type="range" min="0" max="100" v-model="volumePercentage" />
    </main>
</template>
