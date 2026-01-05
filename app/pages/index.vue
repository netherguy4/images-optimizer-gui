<script setup>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

await listen('progress', (event) => {
  const { total, done, current_file } = event.payload;
  const percentage = Math.round((done / total) * 100);

  console.log(`Processing: ${current_file} (${percentage}%)`);
});

const path = ref('');

async function startOptimization() {
  try {
    const result = await invoke('run_optimization', {
      config: {
        paths: [path.value],
        jpg_q: 80,
        png_min: 65,
        png_max: 80,
        webp: true,
        avif: false,
        replace: false,
      },
    });

    console.log('Finished!', result);
  } catch (error) {
    console.error('Optimization failed:', error);
  }
}

const { theme, toggleTheme } = useTheme();
</script>

<template>
  <div class="container">
    <div>
      <input v-model="path" type="text" />
      <button @click="startOptimization">Optimize</button>
    </div>

    <h1>Nuxt 4 Dark Mode</h1>
    <p>Current Theme: {{ theme }}</p>

    <ClientOnly>
      <button class="theme-toggle" @click="toggleTheme">
        Switch to {{ theme === 'dark' ? 'Light' : 'Dark' }}
      </button>

      <template #fallback>
        <button class="theme-toggle" disabled>Loading...</button>
      </template>
    </ClientOnly>
  </div>
</template>
