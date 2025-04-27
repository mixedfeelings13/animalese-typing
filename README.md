# 🎵 Animalese Typing

**Animalese Typing** es una aplicación de escritorio para **Windows** que reproduce sonidos al estilo Animal Crossing mientras escribes. Está desarrollada con **Tauri 2**, **Rust** en el backend y **Svelte** + **Tailwind CSS** en el frontend.

---

## Características principales

- **Captura de teclas en tiempo real**: Emite eventos al presionar letras, números o símbolos.
- **Sonidos estilo Animal Crossing**: Reproduce clips WAV para cada carácter, con carpetas separadas para sonidos comunes y personalidades.
- **Pause / Resume**: Menú de la bandeja y botón in-app para pausar o reanudar la captura.
- **Tray Icon** con menú contextual:
  - **Show**: Restaurar ventana.
  - **Hide**: Ocultar al tray.
  - **Pause**: Alternar captura.
  - **Quit**: Salir de la aplicación.
- **Selector de personalidad**: Diferentes voces de villanos/as de Animal Crossing (smug, lazy, jock, cranky, snooty, sisterly, peppy, normal).
- **Control de volumen** dinámico.
- **Licencia propietaria**: All Rights Reserved (ver `LICENSE`).

---

##  Instalación y desarrollo

1. **Clonar repo**
   ```bash
   git clone https://github.com/pxmpxmpurin/animalese-typing.git
   cd animalese-typing
   ```
2. **Instalar dependencias**
   ```bash
   npm install
   # o yarn
   ```
3. **Desarrollo**
   - **Frontend**:  
     ```bash
     npm run dev
     ```
   - **Tauri** (en otra terminal):  
     ```bash
     npm run tauri dev
     ```

---

## Empaquetado / Build (Windows)

Requisitos:
- Rust (stable) + target `x86_64-pc-windows-msvc`
- Visual Studio Build Tools (C++ workload)

Pasos:
```bash
npm run build        # empaqueta tu frontend
npm run tauri build  # compila Rust y empaqueta el bundle para Windows
```
El ejecutable estará en:
```
src-tauri/target/release/bundle/windows/AnimaleseTyping-<version>-x86_64.exe
```

---

## Configuración

- **Sonidos**: Carpeta `public/sounds/{common,masculine/*,feminine/*}` con archivos `.wav`.

---

## Licencia

Consulta el fichero `LICENSE` (All Rights Reserved).

---

## Contribuciones

¡Las PRs son bienvenidas! Abre un issue o PR con mejoras.


