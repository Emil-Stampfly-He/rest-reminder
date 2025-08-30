// frontend/js/orbit.js
import { CONFIG } from './config.js';

export class OrbitController {
  constructor(container) {
    this.container = container;
    this.circles = Array.from(container.querySelectorAll('.orbit-circle'));
    this.angle = 0;
    this.lastTimestamp = null;
    this.running = true;
  }

  init() {
    const size = (CONFIG.orbitRadiusPx * 2 + CONFIG.circleSizePx);
    this.container.style.position = 'relative';
    this.container.style.width = size + 'px';
    this.container.style.height = size + 'px';
    this.container.style.margin = '40px auto';

    this.circles.forEach((el, idx) => {
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

      // gradient and label set by caller if needed
    });

    requestAnimationFrame(this.loop.bind(this));
  }

  pause() { this.running = false; }
  resume() { this.running = true; }

  updatePositions() {
    const centerX = this.container.clientWidth / 2;
    const centerY = this.container.clientHeight / 2;
    const step = 360 / this.circles.length;
    this.circles.forEach((el, i) => {
      const a = (this.angle + i * step) * (Math.PI / 180);
      const x = centerX + Math.cos(a) * CONFIG.orbitRadiusPx - CONFIG.circleSizePx / 2;
      const y = centerY + Math.sin(a) * CONFIG.orbitRadiusPx - CONFIG.circleSizePx / 2;
      el.style.left = `${x}px`;
      el.style.top = `${y}px`;
    });
  }

  loop(ts) {
    if (this.lastTimestamp === null) this.lastTimestamp = ts;
    const dt = (ts - this.lastTimestamp) / 1000;
    this.lastTimestamp = ts;
    if (this.running) {
      this.angle = (this.angle + CONFIG.rotationSpeedDegPerSec * dt) % 360;
      this.updatePositions();
    }
    requestAnimationFrame(this.loop.bind(this));
  }
}
