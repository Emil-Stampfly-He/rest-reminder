// frontend/js/main.js
import { CONFIG } from './config.js';
import { OrbitController } from './orbit.js';
import { startTyping } from './typing.js';
import { initThemeToggle } from './theme.js';
import { startEmojiBackground } from './emoji-bg.js';

function setupCircles() {
  const container = document.getElementById('orbit-container');
  const circles = Array.from(container.querySelectorAll('.orbit-circle'));
  circles.forEach((el, idx) => {
    const gradStops = CONFIG.instagramGradient.slice();
    for (let i = 0; i < idx; i++) gradStops.push(gradStops.shift());
    const grad = `linear-gradient(135deg, ${gradStops.join(', ')})`;
    el.style.setProperty('--grad', grad);
    el.style.setProperty('--flow-duration', `${6 + idx * 1.5}s`);
    const labelText = el.getAttribute('data-href') || '';
    const span = document.createElement('span');
    span.className = 'label';
    span.textContent = labelText;
    span.style.pointerEvents = 'none';
    el.appendChild(span);

    // keyboard and mouse
    el.addEventListener('keydown', (ev) => {
      if (ev.key === 'Enter' || ev.key === ' ') {
        ev.preventDefault();
        window.location.href = el.getAttribute('data-href');
      }
    });

    el.addEventListener('mouseenter', () => orbit.pause());
    el.addEventListener('mouseleave', () => orbit.resume());
    el.addEventListener('focus', () => orbit.pause());
    el.addEventListener('blur', () => orbit.resume());
    el.addEventListener('click', () => window.location.href = el.getAttribute('data-href'));
  });
}

const orbit = new OrbitController(document.getElementById('orbit-container'));

document.addEventListener('DOMContentLoaded', () => {
  setupCircles();
  orbit.init();
  startTyping();
  initThemeToggle();
  startEmojiBackground();
});
