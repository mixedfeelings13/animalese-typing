<script lang="ts">
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { get, writable } from 'svelte/store';
  import { volume } from '$lib/stores/volume';
  import { isPaused } from '$lib/stores/isPaused';
  import '../app.css';

  const selectedVoice = writable('masculine/smug');

  onMount(() => {
    async function setupListener() {
      const unlisten = await listen<string>('key-pressed', async (event) => {
        if(get(isPaused)) return;

        const input = event.payload.toLowerCase();
        let filename = '';

        if (input === ' ') {
          filename = 'space';
        } else if (input === 'backspace') {
          filename = 'backspace';
        } else if (input === 'enter') {
          filename = 'enter';
        } else if (input === '?') {
          filename = 'question';
        } else if (/^[0-9]$/.test(input)) {
          filename = 'number';
        } else if (/^[a-z]$/.test(input)) {
          filename = input;
        } else {
          filename = 'symbol';
        }

        const isCommonSound = ['space', 'backspace', 'enter', 'question', 'symbol'].includes(filename);
        const voice = $selectedVoice;
        const basePath = isCommonSound ? '/sounds/common' : `/sounds/${voice}`;

        const soundUrl = `${basePath}/${filename}.wav`;

        try {
          const response = await fetch(soundUrl, { method: 'HEAD' });
          if (response.ok) {
            const audio = new Audio(soundUrl);
            audio.volume = Math.min($volume * 0.4, 1);
            await audio.play();
          } else {
            console.warn(`Sound not found: ${soundUrl}`);
          }
        } catch (error) {
          console.error('Sound fetch error', error);
        }
      });

      return () => unlisten();
    }

    setupListener();
  });
</script>

<div class="min-h-screen bg-[url('/patterns/leaves.png')] bg-repeat flex items-center justify-center">
  <div class="w-11/12 max-w-md bg-[#D7C1A0] rounded-2xl shadow-lg p-8 text-center flex flex-col items-center space-y-6">

    <h1 class="text-4xl md:text-5xl font-bold text-[#22543D]">
      🎵 Animalese Typing
    </h1>

    <button class="bg-green-200 text-[#22543D] rounded-full px-6 py-2 hover:bg-green-300 transition-colors duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-green-400"
      on:click={() => isPaused.update(v => !v)}>
      {#if $isPaused}
        Reanudar
      {:else}
        Pausar
      {/if}
    </button>

    <div class="w-full">
      <label for="voiceSelect" class="block mb-2 text-green-900 font-semibold">Selecciona personalidad:</label>
      <select id="voiceSelect" class="w-full bg-green-100 text-[#22543D] rounded-lg px-3 py-2 border border-green-300 focus:outline-none focus:ring-2 focus:ring-green-400" bind:value={$selectedVoice}>
        <optgroup label="Masculino">
          <option value="masculine/smug">Smug</option>
          <option value="masculine/lazy">Lazy</option>
          <option value="masculine/jock">Jock</option>
          <option value="masculine/cranky">Cranky</option>
        </optgroup>
        <optgroup label="Femenino">
          <option value="feminine/snooty">Snooty</option>
          <option value="feminine/sisterly">Sisterly</option>
          <option value="feminine/peppy">Peppy</option>
          <option value="feminine/normal">Normal</option>
        </optgroup>
      </select>
    </div>

    <div class="w-full">
      <label for="volumeRange" class="block mb-2 text-green-900 font-semibold">Volumen:</label>
      <input
      id="volumeRange"
      type="range"
      min="0"
      max="1"
      step="0.01"
      bind:value={$volume}
      class="w-full h-3 rounded-full bg-green-300 appearance-none cursor-pointer focus:outline-none focus:ring-0 accent-green-600
             [&::-webkit-slider-thumb]:appearance-none
             [&::-webkit-slider-thumb]:h-6
             [&::-webkit-slider-thumb]:w-6
             [&::-webkit-slider-thumb]:rounded-full
             [&::-webkit-slider-thumb]:bg-green-700
             [&::-webkit-slider-thumb]:border-2
             [&::-webkit-slider-thumb]:border-green-900
             [&::-webkit-slider-thumb]:shadow-md
             [&::-moz-range-thumb]:appearance-none
      "
      />    
      <p class="text-green-900 mt-2">{Math.round($volume * 100)}%</p>
    </div>

  </div>
</div>
