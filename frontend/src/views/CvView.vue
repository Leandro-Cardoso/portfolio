<template>
  <div class="cv-container">
    <div class="cv-actions no-print">
      <h2>Currículo Profissional</h2>
      <button class="btn-print" @click="printCV">
        🖨️ Imprimir / Exportar PDF
      </button>
    </div>

    <header class="cv-header">
      <div>
        <h1>Leandro Cardoso DEV</h1>
        <p class="subtitle">Engenheiro de Software & Desenvolvedor Fullstack</p>
        <span class="info-line">
          {{ siteInfo.email || 'leandro.dev@exemplo.com' }} • 
          {{ siteInfo.location || 'Brasil' }} • 
          {{ siteInfo.github || 'github.com/leandro-dev' }}
        </span>
      </div>
    </header>

    <section class="cv-card">
      <h3>Resumo Profissional</h3>
      <p>{{ siteInfo.cv_summary || siteInfo.bio || 'Engenheiro de Software focado em sistemas de alta performance com Rust e Vue.js.' }}</p>
    </section>

    <section class="cv-card">
      <h3>Competências Principais</h3>
      <div class="skills-tags">
        <span class="skill-pill">Rust</span>
        <span class="skill-pill">Vue.js 3</span>
        <span class="skill-pill">Engenharia de Software</span>
        <span class="skill-pill">PostgreSQL & SQLite</span>
        <span class="skill-pill">Cloud & DevOps</span>
        <span class="skill-pill">IA & Data Science</span>
      </div>
    </section>

    <section class="cv-card">
      <h3>Projetos de Destaque</h3>
      <div v-for="proj in projects" :key="proj.id" class="cv-item">
        <div class="cv-item-head">
          <strong>{{ proj.title }}</strong>
        </div>
        <p>{{ proj.description }}</p>
        <small class="tech-stack">Tags: {{ proj.tags }}</small>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: 'CvView',
  props: {
    siteInfo: { type: Object, default: () => ({}) },
    projects: { type: Array, default: () => [] }
  },
  methods: {
    printCV() {
      window.print();
    }
  }
};
</script>

<style scoped>
.cv-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.cv-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-md) var(--spacing-lg);
}

.cv-actions h2 {
  font-size: 1.3rem;
  color: var(--text-primary);
}

.btn-print {
  background: var(--color-orange);
  color: #ffffff;
  border: none;
  padding: 10px 18px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-print:hover {
  opacity: 0.9;
}

.cv-header, .cv-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
}

.cv-header h1 {
  font-size: 1.8rem;
  color: var(--text-primary);
}

.subtitle {
  color: var(--color-orange);
  font-weight: 600;
  margin-top: 2px;
}

.info-line {
  font-size: 0.88rem;
  color: var(--text-secondary);
  display: block;
  margin-top: var(--spacing-sm);
}

.cv-card h3 {
  border-bottom: 2px solid var(--border-color);
  padding-bottom: var(--spacing-xs);
  margin-bottom: var(--spacing-md);
  color: var(--text-primary);
}

.skills-tags {
  display: flex;
  gap: var(--spacing-sm);
  flex-wrap: wrap;
}

.skill-pill {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 0.85rem;
  color: var(--text-primary);
}

.cv-item {
  margin-bottom: var(--spacing-md);
  padding-bottom: var(--spacing-sm);
  border-bottom: 1px dashed var(--border-color);
}

.cv-item:last-child {
  border-bottom: none;
}

.cv-item-head strong {
  font-size: 1rem;
  color: var(--text-primary);
}

.tech-stack {
  color: var(--color-mustard);
  font-size: 0.8rem;
  display: block;
  margin-top: 4px;
}
</style>
