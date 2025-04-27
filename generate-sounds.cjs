const fs = require('fs');
const path = require('path');
const animalese = require('@pompompurin/animalese');

const outputDir = path.join(__dirname, 'public', 'sounds');
if (!fs.existsSync(outputDir)) fs.mkdirSync(outputDir, { recursive: true });

const alphabetAndNumbers = 'abcdefghijklmnopqrstuvwxyz0123456789'.split('');

alphabetAndNumbers.forEach(char => {
  const buffer = animalese(char); // correcto aquí
  fs.writeFileSync(path.join(outputDir, `${char}.wav`), buffer);
});

console.log('✅ Sonidos de letras y números generados.');
