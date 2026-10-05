<template>
  <div class="app-layout">
    <Sidebar 
      :currentPath="currentPath" 
      @navigate="handleNavigate" 
      @toggle="handleSidebarToggle"
      class="no-print" 
    />
    
    <main :class="['main-container', { 'collapsed-sidebar': isSidebarCollapsed }]">
      <header class="top-bar no-print">
        <h2>{{ pageTitle }}</h2>
        <div class="status-badge" :class="apiStatus">
          API Rust: {{ apiStatus }}
        </div>
      </header>

      <!-- Views -->
      <HomeView
        v-if="currentPath === '/'"
        :projects="projects"
        :posts="posts"
        :categories="categories"
        :siteInfo="siteInfo"
        @navigate="handleNavigate"
      />

      <AboutView
        v-else-if="currentPath === '/sobre'"
        :siteInfo="siteInfo"
      />

      <CvView
        v-else-if="currentPath === '/curriculo'"
        :siteInfo="siteInfo"
        :projects="projects"
      />

      <ProjectsView
        v-else-if="currentPath === '/projetos'"
        :projects="projects"
        :categories="categories"
      />

      <BlogView
        v-else-if="currentPath === '/blog'"
        :posts="posts"
        :categories="categories"
      />

      <ContactView
        v-else-if="currentPath === '/contato'"
        :siteInfo="siteInfo"
      />

      <AdminView
        v-else-if="currentPath === '/admin'"
        :siteInfo="siteInfo"
        :categories="categories"
        :projects="projects"
        :posts="posts"
        @refresh="fetchData"
      />
    </main>
  </div>
</template>

<script>
import Sidebar from './components/Sidebar.vue';
import HomeView from './views/HomeView.vue';
import AboutView from './views/AboutView.vue';
import CvView from './views/CvView.vue';
import ProjectsView from './views/ProjectsView.vue';
import BlogView from './views/BlogView.vue';
import ContactView from './views/ContactView.vue';
import AdminView from './views/AdminView.vue';

export default {
  name: 'App',
  components: { Sidebar, HomeView, AboutView, CvView, ProjectsView, BlogView, ContactView, AdminView },
  data() {
    return {
      currentPath: '/',
      apiStatus: 'checking...',
      isSidebarCollapsed: JSON.parse(localStorage.getItem('sidebar_collapsed') || 'false'),
      siteInfo: {},
      categories: [],
      projects: [],
      posts: []
    };
  },
  computed: {
    pageTitle() {
      const titles = {
        '/': 'Dashboard / Home',
        '/sobre': 'Sobre Mim',
        '/curriculo': 'Currículo Profissional',
        '/projetos': 'Projetos',
        '/blog': 'Blog',
        '/contato': 'Contato',
        '/admin': 'Painel Admin'
      };
      return titles[this.currentPath] || 'Leandro Dev';
    }
  },
  mounted() {
    this.fetchData();
  },
  methods: {
    handleNavigate(path) {
      this.currentPath = path;
    },
    handleSidebarToggle(collapsed) {
      this.isSidebarCollapsed = collapsed;
    },
    async fetchData() {
      try {
        const [resInfo, resCat, resProj, resBlog] = await Promise.all([
          fetch('http://localhost:8080/api/v1/site-info'),
          fetch('http://localhost:8080/api/v1/categories'),
          fetch('http://localhost:8080/api/v1/projects'),
          fetch('http://localhost:8080/api/v1/blog')
        ]);

        if (resInfo.ok && resCat.ok && resProj.ok && resBlog.ok) {
          this.siteInfo = await resInfo.json();
          this.categories = await resCat.json();
          this.projects = await resProj.json();
          this.posts = await resBlog.json();
          this.apiStatus = 'online';
        } else {
          this.apiStatus = 'offline';
        }
      } catch (err) {
        console.error('Erro ao conectar com backend Rust:', err);
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
  width: calc(100% - var(--sidebar-width));
}

.main-container.collapsed-sidebar {
  margin-left: var(--sidebar-collapsed-width);
  width: calc(100% - var(--sidebar-collapsed-width));
}

@media (max-width: 768px) {
  .main-container {
    margin-left: var(--sidebar-collapsed-width);
    width: calc(100% - var(--sidebar-collapsed-width));
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
</style>
