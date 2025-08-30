// frontend/js/emoji-bg.js
export function startEmojiBackground(containerId = 'emoji-bg') {
  const container = document.getElementById(containerId);
  if (!container) return;
  const EMOJI = '⏰';
  const spawnIntervalMs = 600;

  function spawnOne() {
    const el = document.createElement('div');
    el.className = 'emoji-fall';
    el.textContent = EMOJI;
    const left = Math.random() * 100;
    el.style.left = left + '%';
    const size = 18 + Math.floor(Math.random() * 28);
    el.style.fontSize = size + 'px';
    const duration = 5000 + Math.floor(Math.random() * 8000);
    el.style.setProperty('--duration', duration + 'ms');
    el.style.animationDuration = duration + 'ms';
    const drift = Math.round((Math.random() - 0.5) * 120);
    el.style.setProperty('--drift', drift + 'px');
    el.style.top = '-10%';
    container.appendChild(el);
    setTimeout(() => el.remove(), duration + 2000);
  }

  setInterval(() => {
    const count = Math.random() < 0.6 ? 1 : Math.random() < 0.3 ? 2 : 0;
    for (let i = 0; i < count; i++) setTimeout(spawnOne, Math.random() * 800);
  }, spawnIntervalMs);
}
