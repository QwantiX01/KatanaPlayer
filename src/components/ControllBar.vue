<script setup lang="ts">
import Backdrop from "./ui/Backdrop.vue";
</script>

<template>
  <backdrop variant="dark" :roundness="22" :height="96" class="bar">
    <!-- Секція 1: Обкладинка альбому -->
    <div class="preview">
      <img src="" alt="Kill Each Other / Live Forever" class="album-cover">
    </div>

    <!-- Секція 2: Основні контроли -->
    <div class="main-section">
      <div class="top-row">
        <div class="metadata">
          <div class="title">Kill Each Other / Live Forever</div>
          <div class="artist">Scars On Broadway</div>
        </div>
        <div class="playback-controls">
          <button class="control-btn">1</button>
          <button class="control-btn play-btn">2</button>
          <button class="control-btn">3</button>
        </div>
      </div>
      <div class="timeline-info">
        <p class="time">0:05 / 3:05</p>
        <input class="timeline" type="range" min="0" max="185" value="5">
      </div>
    </div>

    <!-- Секція 3: Додаткові контроли та гучність -->
    <div class="right-section">
      <div class="extra-controls">
        <button class="control-btn">1</button>
        <button class="control-btn">2</button>
        <button class="control-btn">3</button>
      </div>
      <div class="volume-controls">
        <input class="volume-slider" type="range" min="0" max="100" value="75">
        <p class="time">0</p>
      </div>
    </div>
  </backdrop>
</template>

<style lang="css" scoped>
.bar {
  box-sizing: border-box;
  display: flex;
  padding: 1rem;
  gap: 1.5rem;
  align-items: center;
}

/* Секція 1: Обкладинка */
.preview {
  flex-shrink: 0;
}

.album-cover {
  width: 64px;
  height: 64px;
  border-radius: 8px;
  object-fit: cover;
}

/* Секція 2: Основні контроли */
.main-section {
  justify-content: space-between;
  height: 100%;
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.top-row {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  position: relative;
}

.metadata {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  width: max-content;
  min-width: 0;


}

.title {
  color: var(--primary-text);
  font-size: 1.1rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.artist {
  color: var(--secondary-text);
  font-size: 0.8rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.playback-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin: 0 auto;
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

.timeline-info {
  display: flex;
  gap: 1rem;
}

.time {
  color: #aaa;
  font-size: 0.8rem;
  white-space: nowrap;
  flex-shrink: 0;
}

.timeline {
  flex: 1;
  height: 4px;
  background: #333;
  border-radius: 2px;
  outline: none;
  cursor: pointer;
}

/* Секція 3: Додаткові контроли */
.right-section {
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  min-width: 0;
  align-items: flex-end;
  flex-shrink: 0;
}

.extra-controls {
  display: flex;
  width: 100%;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
}

.volume-controls {
  display: flex;
  gap: 0.5rem;
}

.volume-slider {
  width: 100%;
  height: 4px;
  background: #333;
  border-radius: 2px;
  outline: none;
  cursor: pointer;
}

.control-btn {
  background: transparent;
  border: none;
  color: #aaa;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 4px;
  font-size: 1rem;
  transition: color 0.2s ease;
}

.control-btn:hover {
  color: white;
}

.play-btn {
  color: white;
  font-size: 1.2rem;
}

/* Стилі для слайдерів */
.timeline::-webkit-slider-thumb,
.volume-slider::-webkit-slider-thumb {
  appearance: none;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: white;
  cursor: pointer;
}

.timeline::-moz-range-thumb,
.volume-slider::-moz-range-thumb {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: white;
  cursor: pointer;
  border: none;
}
</style>