// frontend/js/app.js
// Implements three orbiting circles that rotate slowly around the center
// Requirements satisfied:
// 1) Three circles orbit around a center point.
// 2) Hovering any circle pauses the overall rotation and visually selects
//    the hovered circle (it shrinks to indicate selection).
// 3) Colors follow an Instagram-like gradient palette.
// 4) All JS is located in frontend/js/app.js and fully commented in English.
// 5) Clicking a circle will navigate to an URL provided in its
//    `data-href` attribute (placeholder left for the caller).
// 6) Code organized for maintainability and clarity.

(function () {
  'use strict';

  // Config: makes it easy to change behavior
  const CONFIG = {
  rotationSpeedDegPerSec: 8, // degrees per second (slow rotation)
  orbitRadiusPx: 140, // radius of the orbit
  circleSizePx: 190, // base diameter (increased size)
  shrinkFactor: 0.9, // how much the selected circle shrinks
    instagramGradient: [
      '#feda75', // orange
      '#fa7e1e', // orange-red
      '#d62976', // magenta
      '#962fbf', // purple
      '#4f5bd5'  // blue
    ]
  };

  // DOM references
  const container = document.getElementById('orbit-container');
  const circles = Array.from(container.querySelectorAll('.orbit-circle'));

  // internal state
  let angle = 0; // global rotation angle in degrees
  let lastTimestamp = null;
  let running = true; // whether the rotation is active

  // Apply initial styling and colors
  function init() {
    container.style.position = 'relative';
    container.style.width = (CONFIG.orbitRadiusPx * 2 + CONFIG.circleSizePx) + 'px';
    container.style.height = (CONFIG.orbitRadiusPx * 2 + CONFIG.circleSizePx) + 'px';

    // center container visually
    container.style.margin = '40px auto';

    circles.forEach((el, idx) => {
      // set size and absolute positioning will be handled by updatePositions
      el.style.width = CONFIG.circleSizePx + 'px';
      el.style.height = CONFIG.circleSizePx + 'px';
      el.style.borderRadius = '50%';
      el.style.position = 'absolute';
      el.style.transformOrigin = 'center center';
      el.style.display = 'flex';
      el.style.alignItems = 'center';
      el.style.justifyContent = 'center';
      el.style.cursor = 'pointer';
      el.style.boxShadow = '0 8px 20px rgba(0,0,0,0.15)';
      el.style.transition = 'transform 220ms cubic-bezier(.2,.8,.2,1), box-shadow 220ms';

  // Set animated gradient via a CSS custom property so CSS can animate it.
  // We rotate the color stops a bit per circle to give visual variety.
  const gradStops = rotateArray(CONFIG.instagramGradient, idx);
  const grad = `linear-gradient(135deg, ${gradStops.join(', ')})`;
  el.style.setProperty('--grad', grad);
  // give each circle a slightly different animation duration for organic feel
  el.style.setProperty('--flow-duration', `${6 + idx * 1.5}s`);

  // Place label text from data-href inside the circle (shows selected text)
  // Create a dedicated span so it sits above the animated background
  const labelText = el.getAttribute('data-href') || '';
  const span = document.createElement('span');
  span.className = 'label';
  span.textContent = labelText;
  // Make the label non-interactive (clicks handled by parent)
  span.style.pointerEvents = 'none';
  el.appendChild(span);

      // Accessibility: allow keyboard activation
      el.addEventListener('keydown', (ev) => {
        if (ev.key === 'Enter' || ev.key === ' ') {
          ev.preventDefault();
          triggerClick(el);
        }
      });

      // Hover/Focus handlers
      el.addEventListener('mouseenter', () => onHoverStart(el));
      el.addEventListener('mouseleave', () => onHoverEnd(el));
      el.addEventListener('focus', () => onHoverStart(el));
      el.addEventListener('blur', () => onHoverEnd(el));

      // Click handler -> navigation placeholder
      el.addEventListener('click', () => triggerClick(el));
    });

    // kick off animation loop
    requestAnimationFrame(loop);
  }

  // rotate an array by n positions
  function rotateArray(arr, n) {
    const copy = arr.slice();
    for (let i = 0; i < n; i++) copy.push(copy.shift());
    return copy;
  }

  // pause rotation and shrink the hovered element
  function onHoverStart(el) {
    running = false; // pause global rotation

    // shrink selected
    el.style.transform = `scale(${CONFIG.shrinkFactor})`;
    el.style.boxShadow = '0 12px 30px rgba(0,0,0,0.25)';
  }

  // resume rotation and restore size
  function onHoverEnd(el) {
    running = true; // resume rotation
    el.style.transform = 'scale(1)';
    el.style.boxShadow = '0 8px 20px rgba(0,0,0,0.15)';
  }

  // click action placeholder: navigate to the URL in data-href
  function triggerClick(el) {
    const href = el.getAttribute('data-href');
    if (href) {
      // Navigation is left as a simple page change. Caller may replace
      // with router logic or other SPA behavior.
      window.location.href = href;
    }
  }

  // update positions for each circle based on the global angle
  function updatePositions() {
    const centerX = container.clientWidth / 2;
    const centerY = container.clientHeight / 2;

    // distribute circles evenly on the orbit (120deg apart)
    const step = 360 / circles.length;

    circles.forEach((el, i) => {
      const a = (angle + i * step) * (Math.PI / 180); // to radians
      const x = centerX + Math.cos(a) * CONFIG.orbitRadiusPx - CONFIG.circleSizePx / 2;
      const y = centerY + Math.sin(a) * CONFIG.orbitRadiusPx - CONFIG.circleSizePx / 2;
      el.style.left = `${x}px`;
      el.style.top = `${y}px`;
    });
  }

  // animation loop: advances angle when running
  function loop(ts) {
    if (lastTimestamp === null) lastTimestamp = ts;
    const dt = (ts - lastTimestamp) / 1000; // seconds since last frame
    lastTimestamp = ts;

    if (running) {
      // update angle using config speed
      angle = (angle + CONFIG.rotationSpeedDegPerSec * dt) % 360;
      updatePositions();
    }

    requestAnimationFrame(loop);
  }

  // initialize DOM & start animation
  init();

  // Typing animation for the main title -> uses the #main-title element's
  // data-text attribute and types it one letter at a time.
  (function typingTitle() {
    const el = document.getElementById('main-title');
    if (!el) return;
    const text = el.getAttribute('data-text') || '';
    el.textContent = '';
    let i = 0;
    const delay = 60; // ms per character

    function step() {
      if (i <= text.length) {
        el.textContent = text.slice(0, i);
        i++;
        setTimeout(step, delay + Math.random() * 80);
      }
    }
    // start after a short delay so rest of page paints
    setTimeout(step, 250);
  })();

  // Theme toggle: toggles data-theme on root and persists choice to localStorage
  (function themeToggle() {
    const btn = document.getElementById('theme-toggle');
    if (!btn) return;

    const ROOT = document.documentElement;
    const STORAGE_KEY = 'rest-reminder-theme';

    function applyTheme(theme) {
      ROOT.setAttribute('data-theme', theme);
      btn.setAttribute('aria-pressed', theme === 'dark' ? 'true' : 'false');
      // update icon
      btn.textContent = theme === 'dark' ? '☀️' : '🌙';
    }

    // load stored or prefer dark if user prefers
    const stored = localStorage.getItem(STORAGE_KEY);
    const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    applyTheme(stored || (prefersDark ? 'dark' : 'light'));

    btn.addEventListener('click', () => {
      const current = ROOT.getAttribute('data-theme') === 'dark' ? 'dark' : 'light';
      const next = current === 'dark' ? 'light' : 'dark';
      applyTheme(next);
      localStorage.setItem(STORAGE_KEY, next);
    });
  })();
})();
