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
            audio.volume = $volume;
            audio.play().catch((e) => console.error('Audio play error', e));
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

<main class="container">
  <h1>Animalese Typing 🎶</h1>

  <button on:click={() => isPaused.update(v => !v)}>
    {#if $isPaused}
      Reanudar
    {:else}
      Pausar
   {/if}
  </button>

  <label for="voiceSelect">Selecciona personalidad:</label>
  <select id="voiceSelect" bind:value={$selectedVoice}>
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

  <div class="volume-control">
    <label for="volumeRange">Volumen:</label>
    <input
      id="volumeRange"
      type="range"
      min="0"
      max="1"
      step="0.01"
      bind:value={$volume}
    />
    <span>{Math.round($volume * 100)}%</span>
  </div>
</main>

<style>
  main.container {
    padding: 2rem;
    max-width: 600px;
    margin: 0 auto;
    text-align: center;
  }

  h1 {
    font-size: 2rem;
    margin-bottom: 2rem;
  }

  select, input[type="range"] {
    margin-top: 1rem;
    width: 80%;
    padding: 0.5rem;
    font-size: 1rem;
  }

  .volume-control {
    margin-top: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
  }

  input[type="range"] {
    cursor: pointer;
  }
</style>
