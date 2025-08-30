// frontend/js/theme.js
export function initThemeToggle(buttonId = 'theme-toggle') {
  const btn = document.getElementById(buttonId);
  if (!btn) return;
  const ROOT = document.documentElement;
  const STORAGE_KEY = 'rest-reminder-theme';

  function applyTheme(theme) {
    ROOT.setAttribute('data-theme', theme);
    btn.setAttribute('aria-pressed', theme === 'dark' ? 'true' : 'false');
    btn.textContent = theme === 'dark' ? '☀️' : '🌙';
  }

  const stored = localStorage.getItem(STORAGE_KEY);
  const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
  applyTheme(stored || (prefersDark ? 'dark' : 'light'));

  btn.addEventListener('click', () => {
    const current = ROOT.getAttribute('data-theme') === 'dark' ? 'dark' : 'light';
    const next = current === 'dark' ? 'light' : 'dark';
    applyTheme(next);
    localStorage.setItem(STORAGE_KEY, next);
  });
}
