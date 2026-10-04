<template>
  <div class="home-container">
    <!-- Hero / Apresentação -->
    <section class="hero-card">
      <div class="hero-content">
        <span class="badge-role">Engenheiro de Software & Fullstack Developer</span>
        <h1>Leandro Cardoso DEV</h1>
        <p class="hero-bio">
          Especialista em arquitetura de sistemas, desenvolvimento fullstack (Rust & Vue), infraestrutura Cloud, Banco de Dados, Engenharia de Dados e IA/Machine Learning.
        </p>
        <div class="hero-actions no-print">
          <button class="btn-primary" @click="$emit('navigate', '/projetos')">Ver Projetos 🚀</button>
          <button class="btn-secondary" @click="printResume">Imprimir / Salvar PDF 📄</button>
        </div>
      </div>
    </section>

    <!-- Dashboard / Métricas em Cards Menores -->
    <section class="metrics-grid">
      <div class="metric-card">
        <span class="metric-icon">🚀</span>
        <div class="metric-data">
          <h4>Projetos Ativos</h4>
          <span class="metric-value">{{ projects.length }}</span>
        </div>
      </div>
      <div class="metric-card">
        <span class="metric-icon">📝</span>
        <div class="metric-data">
          <h4>Artigos Publicados</h4>
          <span class="metric-value">{{ posts.length }}</span>
        </div>
      </div>
      <div class="metric-card">
        <span class="metric-icon">⚡</span>
        <div class="metric-data">
          <h4>Core Stack</h4>
          <span class="metric-value">Rust & Vue.js</span>
        </div>
      </div>
      <div class="metric-card">
        <span class="metric-icon">🟢</span>
        <div class="metric-data">
          <h4>Status do Sistema</h4>
          <span class="metric-value status-online">Operacional</span>
        </div>
      </div>
    </section>

    <!-- Áreas de Especialidade -->
    <section class="skills-card">
      <h3>Especialidades & Conhecimentos</h3>
      <div class="skills-grid">
        <div class="skill-item">
          <strong>Engenharia de Software</strong>
          <p>Arquitetura limpa, design patterns e desenvolvimento de engines.</p>
        </div>
        <div class="skill-item">
          <strong>Cloud & Infraestrutura</strong>
          <p>Containers, pipelines CI/CD, backup automatizado e object storage.</p>
        </div>
        <div class="skill-item">
          <strong>Banco de Dados & APIs</strong>
          <p>PostgreSQL, APIs RESTful de alta performance em Rust puro e Actix/Axum.</p>
        </div>
        <div class="skill-item">
          <strong>IA & Data Science</strong>
          <p>Machine Learning, análise de dados e automação inteligente.</p>
        </div>
      </div>
    </section>

    <!-- Últimos Projetos -->
    <section class="recent-section">
      <h3>Destaques de Projetos</h3>
      <div class="cards-grid">
        <ItemCard
          v-for="project in projects.slice(0, 2)"
          :key="project.id"
          :title="project.title"
          :description="project.description"
          :category="getCategory(project.category_id)"
          :tags="project.tags"
          :link="project.link"
        />
      </div>
    </section>
  </div>
</template>

<script>
import ItemCard from '../components/ItemCard.vue';

export default {
  name: 'HomeView',
  components: { ItemCard },
  props: {
    projects: { type: Array, default: () => [] },
    posts: { type: Array, default: () => [] },
    categories: { type: Array, default: () => [] }
  },
  methods: {
    getCategory(catId) {
      return this.categories.find(c => c.id === catId);
    },
    printResume() {
      window.print();
    }
  }
};
</script>

<style scoped>
.home-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
}

.hero-card {
  background: linear-gradient(135deg, var(--bg-surface) 0%, rgba(249, 115, 22, 0.05) 100%);
  border: 1px solid var(--border-color);
  border-left: 6px solid var(--color-orange);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  box-shadow: var(--shadow-sm);
}

.badge-role {
  color: var(--color-mustard);
  background-color: rgba(234, 179, 8, 0.15);
  font-size: 0.8rem;
  font-weight: 700;
  padding: 4px 10px;
  border-radius: 12px;
}

.hero-card h1 {
  font-size: 2.2rem;
  margin: var(--spacing-sm) 0;
  color: var(--text-primary);
}

.hero-bio {
  color: var(--text-secondary);
  font-size: 1.05rem;
  line-height: 1.6;
  max-width: 800px;
  margin-bottom: var(--spacing-lg);
}

.hero-actions {
  display: flex;
  gap: var(--spacing-md);
}

.btn-primary {
  background-color: var(--color-orange);
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: var(--radius-md);
  font-weight: 600;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background-color: var(--color-orange-hover);
}

.btn-secondary {
  background: transparent;
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  padding: 10px 20px;
  border-radius: var(--radius-md);
  font-weight: 600;
  cursor: pointer;
}

.btn-secondary:hover {
  background: var(--bg-primary);
}

/* Metrics Grid */
.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-md);
}

.metric-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-md);
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.metric-icon {
  font-size: 1.8rem;
}

.metric-data h4 {
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.metric-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--text-primary);
}

.status-online {
  color: var(--color-success);
}

/* Skills */
.skills-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
}

.skills-card h3 {
  margin-bottom: var(--spacing-lg);
}

.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: var(--spacing-lg);
}

.skill-item strong {
  color: var(--color-orange);
  display: block;
  margin-bottom: var(--spacing-xs);
}

.skill-item p {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

/* Recent Section */
.recent-section h3 {
  margin-bottom: var(--spacing-md);
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-md);
}
</style>
