// frontend/js/typing.js
export function startTyping(id = 'main-title', delayStart = 250) {
  const el = document.getElementById(id);
  if (!el) return;
  const text = el.getAttribute('data-text') || '';
  el.textContent = '';
  let i = 0;
  const delay = 60;
  function step() {
    if (i <= text.length) {
      el.textContent = text.slice(0, i);
      i++;
      setTimeout(step, delay + Math.random() * 80);
    }
  }
  setTimeout(step, delayStart);
}
