<template>
  <div class="about-container">
    <!-- Header de Perfil -->
    <section class="profile-card">
      <div class="profile-header">
        <div class="avatar-box">
          <span class="avatar-initials">LC</span>
        </div>
        <div class="profile-info">
          <h2>Leandro Cardoso DEV</h2>
          <p class="role-subtitle">Engenheiro de Software & Fullstack Developer</p>
          <div class="location-badge">📍 Brasil</div>
        </div>
      </div>
      <p class="profile-bio">
        Engenheiro de Software focado em construir sistemas resilientes, APIs de alta performance com Rust e interfaces web modernas e reativas com Vue.js. Atuo desde a modelagem de banco de dados e infraestrutura Cloud até a implementação de modelos de IA e Ciência de Dados.
      </p>
    </section>

    <!-- Pilares Técnicos -->
    <section class="pillars-grid">
      <div class="pillar-card">
        <span class="pillar-icon">⚙️</span>
        <h3>Engenharia & Backend</h3>
        <p>Desenvolvimento de APIs REST com Rust (Actix/Axum), arquitetura limpa, microsserviços, concorrência e engines 2D personalizadas.</p>
      </div>

      <div class="pillar-card">
        <span class="pillar-icon">🎨</span>
        <h3>Frontend Moderno</h3>
        <p>Interfaces reativas e modulares com Vue 3, consumo de APIs REST, CSS totalmente customizado (Design Systems) e UX otimizada.</p>
      </div>

      <div class="pillar-card">
        <span class="pillar-icon">☁️</span>
        <h3>Cloud & DevOps</h3>
        <p>Infraestrutura própria e Cloud (Hetzner, Cloudflare, S3 B2), automação de pipelines CI/CD via Git Push e rotinas de backup.</p>
      </div>

      <div class="pillar-card">
        <span class="pillar-icon">📊</span>
        <h3>IA & Data Science</h3>
        <p>Modelagem de banco de dados SQL/PostgreSQL, análise estatística de dados e aplicação prática de Machine Learning.</p>
      </div>
    </section>

    <!-- Histórico & Formação -->
    <section class="experience-card">
      <h3>Destaques de Atuação</h3>
      <div class="timeline-list">
        <div class="timeline-item">
          <div class="timeline-marker"></div>
          <div class="timeline-content">
            <h4>Tiles Gear Engine</h4>
            <span class="timeline-date">Projeto Open Source</span>
            <p>Desenvolvimento de uma engine de jogos 2D modular em Rust focada em arquitetura de camadas e tiles.</p>
          </div>
        </div>

        <div class="timeline-item">
          <div class="timeline-marker"></div>
          <div class="timeline-content">
            <h4>Sistemas Web & Cloud Pipelines</h4>
            <span class="timeline-date">Fullstack & DevOps</span>
            <p>Construção de aplicações completas integrando Django, Vue.js, PostgreSQL e infraestrutura auto-hospedada.</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: 'AboutView'
};
</script>

<style scoped>
.about-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
}

.profile-card, .experience-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
}

.profile-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
}

.avatar-box {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  background: linear-gradient(135deg, var(--color-orange), var(--color-mustard));
  display: flex;
  align-items: center;
  justify-content: center;
}

.avatar-initials {
  color: #fff;
  font-weight: 800;
  font-size: 1.5rem;
}

.profile-info h2 {
  font-size: 1.6rem;
  color: var(--text-primary);
}

.role-subtitle {
  color: var(--color-orange);
  font-weight: 600;
  font-size: 0.95rem;
}

.location-badge {
  font-size: 0.8rem;
  color: var(--text-secondary);
  margin-top: 2px;
}

.profile-bio {
  color: var(--text-secondary);
  line-height: 1.6;
  font-size: 1rem;
}

.pillars-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: var(--spacing-lg);
}

.pillar-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
  transition: transform 0.2s;
}

.pillar-card:hover {
  transform: translateY(-2px);
}

.pillar-icon {
  font-size: 1.8rem;
  display: block;
  margin-bottom: var(--spacing-sm);
}

.pillar-card h3 {
  font-size: 1.1rem;
  margin-bottom: var(--spacing-xs);
  color: var(--text-primary);
}

.pillar-card p {
  font-size: 0.88rem;
  color: var(--text-secondary);
  line-height: 1.5;
}

.timeline-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
  margin-top: var(--spacing-lg);
  padding-left: var(--spacing-md);
  border-left: 2px solid var(--border-color);
}

.timeline-item {
  position: relative;
  padding-left: var(--spacing-md);
}

.timeline-marker {
  position: absolute;
  left: -23px;
  top: 4px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--color-orange);
}

.timeline-content h4 {
  color: var(--text-primary);
  font-size: 1rem;
}

.timeline-date {
  font-size: 0.8rem;
  color: var(--color-mustard);
  font-weight: 600;
}

.timeline-content p {
  font-size: 0.9rem;
  color: var(--text-secondary);
  margin-top: 4px;
}
</style>
