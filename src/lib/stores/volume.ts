import { writable } from 'svelte/store';

// Cargamos volumen guardado o usamos 0.4 como predeterminado
const savedVolume = localStorage.getItem('animalese-typing-volume');
const initialVolume = savedVolume ? parseFloat(savedVolume) : 0.4;

// Creamos el store
export const volume = writable(initialVolume);

// Cada vez que cambie, lo guardamos en localStorage
volume.subscribe((value) => {
  localStorage.setItem('animalese-typing-volume', value.toString());
});
