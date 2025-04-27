<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from '@tauri-apps/api/event';
  import { writable } from 'svelte/store';

  //Estado
  const selectedVoice = writable('masculine/smug');

  onMount(() => {
  async function setupListener() {
    const unlisten = await listen<string>('key-pressed', (event) => {
      const letter = event.payload;

      selectedVoice.subscribe((voice) => {
        const filename = `/sounds/${voice}/${letter}.wav`;
        const audio = new Audio(filename);
        audio.play();
      });
    });

    return () => unlisten();
  }

  setupListener();
  });

</script>

<main class="container">
  <h1>Animalese typing</h1>
  
  <label for="voice-select">Selecciona personalidad:</label>
  <select id="voice-select" bind:value={$selectedVoice}>
    <optgroup label="Masculino">
      <option value="masculine/smug">Smug</option>
      <option value="masculine/lazy">Lazy</option>
      <option value="masculine/jock">Jock</option>
      <option value="masculine/cranky">Cranky</option>
    </optgroup>

    <optgroup label="Femenino">
      <option value="feminine/snooty">Snooty</option>
      <option value="feminine/peppy">Peppy</option>
      <option value="feminine/sisterly">Sisterly</option>
      <option value="feminine/normal">Normal</option>
    </optgroup>
  </select>
</main>

<style>
  main {
    text-align: center;
    padding: 2rem;
    font-family: 'Segoe UI', sans-serif;
  }

  select {
    margin-top: 1rem;
    padding: 0.5rem;
    font-size: 1rem;
  }

</style>
