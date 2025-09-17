<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, watch } from "vue";
import ControllBar from "./components/ControllBar.vue";
import Backdrop from "./components/ui/Backdrop.vue";

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
    <main class="application">
        <Backdrop variant="dark" :roundness="22"></Backdrop>
        <ControllBar></ControllBar>
    </main>
</template>

<style>
.application {
    box-sizing: border-box;
    height: 100vh;
    width: 100vw;
    padding: 1rem;
    gap: 1.5rem;
    background-color: var(--background-dark);
    display: flex;
    flex-direction: column;
}

.bob {
    background-color: var(--background-primary);
    flex: 1;
}
</style>
