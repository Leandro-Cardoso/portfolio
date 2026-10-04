<template>
  <div class="app-layout">
    <Sidebar :currentPath="currentPath" @navigate="handleNavigate" class="no-print" />
    
    <main class="main-container">
      <header class="top-bar">
        <h2>{{ pageTitle }}</h2>
        <div class="status-badge" :class="apiStatus">
          API Rust: {{ apiStatus }}
        </div>
      </header>

      <section class="content-card">
        <p>Bem-vindo à estrutura base. O ambiente está configurado e pronto para o desenvolvimento das próximas etapas.</p>
      </section>
    </main>
  </div>
</template>

<script>
import Sidebar from './components/Sidebar.vue';

export default {
  name: 'App',
  components: { Sidebar },
  data() {
    return {
      currentPath: '/',
      apiStatus: 'checking...'
    };
  },
  computed: {
    pageTitle() {
      const titles = {
        '/': 'Home / Dashboard',
        '/sobre': 'Sobre Mim',
        '/projetos': 'Projetos',
        '/blog': 'Blog',
        '/contato': 'Contato',
        '/admin': 'Painel Administrativo'
      };
      return titles[this.currentPath] || 'Leandro Dev';
    }
  },
  mounted() {
    this.checkApiHealth();
  },
  methods: {
    handleNavigate(path) {
      this.currentPath = path;
    },
    async checkApiHealth() {
      try {
        const res = await fetch('http://localhost:8080/api/v1/health');
        if (res.ok) {
          this.apiStatus = 'online';
        } else {
          this.apiStatus = 'offline';
        }
      } catch {
        this.apiStatus = 'offline';
      }
    }
  }
};
</script>

<style scoped>
.app-layout {
  display: flex;
  min-height: 100vh;
}

.main-container {
  flex: 1;
  margin-left: var(--sidebar-width);
  padding: var(--spacing-xl);
  transition: margin-left var(--transition-speed) ease;
}

@media (max-width: 768px) {
  .main-container {
    margin-left: var(--sidebar-collapsed-width);
    padding: var(--spacing-md);
  }
}

.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-lg);
}

.status-badge {
  padding: 4px 12px;
  border-radius: var(--radius-sm);
  font-size: 0.85rem;
  font-weight: 600;
}

.status-badge.online {
  background-color: rgba(34, 197, 94, 0.2);
  color: var(--color-success);
}

.status-badge.offline {
  background-color: rgba(239, 68, 68, 0.2);
  color: var(--color-error);
}

.content-card {
  background-color: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
  box-shadow: var(--shadow-sm);
}
</style>
