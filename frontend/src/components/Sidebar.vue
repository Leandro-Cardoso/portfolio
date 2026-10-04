<template>
  <aside :class="['sidebar', { collapsed: isCollapsed }]">
    <div class="sidebar-header">
      <div class="logo-box">
        <span class="logo-badge">LC</span>
        <span v-if="!isCollapsed" class="logo-text">Leandro Dev</span>
      </div>
      <button class="toggle-btn" @click="toggleSidebar" :title="isCollapsed ? 'Expandir' : 'Recolher'">
        {{ isCollapsed ? '❯' : '❮' }}
      </button>
    </div>

    <nav class="sidebar-nav">
      <ul>
        <li v-for="item in navItems" :key="item.path">
          <a
            :href="item.path"
            :class="['nav-item', { active: currentPath === item.path }]"
            @click.prevent="navigate(item.path)"
          >
            <span class="icon">{{ item.icon }}</span>
            <span v-if="!isCollapsed" class="label">{{ item.label }}</span>
          </a>
        </li>
      </ul>
    </nav>

    <div class="sidebar-footer">
      <button class="theme-toggle" @click="cycleTheme" :title="'Tema atual: ' + currentTheme">
        <span class="icon">{{ themeIcon }}</span>
        <span v-if="!isCollapsed" class="label">{{ currentThemeLabel }}</span>
      </button>
    </div>
  </aside>
</template>

<script>
export default {
  name: 'Sidebar',
  props: {
    currentPath: { type: String, default: '/' }
  },
  data() {
    return {
      isCollapsed: JSON.parse(localStorage.getItem('sidebar_collapsed') || 'false'),
      currentTheme: localStorage.getItem('app_theme') || 'system',
      navItems: [
        { path: '/', label: 'Home', icon: '🏠' },
        { path: '/sobre', label: 'Sobre', icon: '👤' },
        { path: '/projetos', label: 'Projetos', icon: '🚀' },
        { path: '/blog', label: 'Blog', icon: '📝' },
        { path: '/contato', label: 'Contato', icon: '✉️' },
        { path: '/admin', label: 'Admin', icon: '⚙️' }
      ]
    };
  },
  computed: {
    themeIcon() {
      if (this.currentTheme === 'light') return '☀️';
      if (this.currentTheme === 'dark') return '🌙';
      return '🖥️';
    },
    currentThemeLabel() {
      if (this.currentTheme === 'light') return 'Claro';
      if (this.currentTheme === 'dark') return 'Escuro';
      return 'Sistema';
    }
  },
  mounted() {
    this.applyTheme(this.currentTheme);
  },
  methods: {
    toggleSidebar() {
      this.isCollapsed = !this.isCollapsed;
      localStorage.setItem('sidebar_collapsed', JSON.stringify(this.isCollapsed));
      this.$emit('toggle', this.isCollapsed);
    },
    cycleTheme() {
      const themes = ['system', 'light', 'dark'];
      const nextIndex = (themes.indexOf(this.currentTheme) + 1) % themes.length;
      this.currentTheme = themes[nextIndex];
      localStorage.setItem('app_theme', this.currentTheme);
      this.applyTheme(this.currentTheme);
    },
    applyTheme(theme) {
      if (theme === 'system') {
        document.documentElement.removeAttribute('data-theme');
      } else {
        document.documentElement.setAttribute('data-theme', theme);
      }
    },
    navigate(path) {
      this.$emit('navigate', path);
    }
  }
};
</script>

<style scoped>
.sidebar {
  width: var(--sidebar-width);
  height: 100vh;
  background-color: var(--bg-sidebar);
  color: var(--text-sidebar);
  display: flex;
  flex-direction: column;
  transition: width var(--transition-speed) ease;
  position: fixed;
  left: 0;
  top: 0;
  z-index: 100;
  border-right: 1px solid var(--border-color);
}

.sidebar.collapsed {
  width: var(--sidebar-collapsed-width);
}

.sidebar-header {
  padding: var(--spacing-md);
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo-box {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.logo-badge {
  background: linear-gradient(135deg, var(--color-mustard), var(--color-orange));
  color: var(--color-black);
  font-weight: bold;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
}

.logo-text {
  color: #fff;
  font-weight: 700;
  white-space: nowrap;
}

.toggle-btn, .theme-toggle {
  background: transparent;
  border: none;
  color: var(--text-sidebar);
  cursor: pointer;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  transition: background var(--transition-speed);
}

.toggle-btn:hover, .theme-toggle:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fff;
}

.sidebar-nav {
  flex: 1;
  padding: var(--spacing-md) 0;
}

.sidebar-nav ul {
  list-style: none;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: var(--spacing-md);
  color: var(--text-sidebar);
  text-decoration: none;
  gap: var(--spacing-md);
  border-left: 4px solid transparent;
  transition: all 0.2s;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: #fff;
}

.nav-item.active {
  border-left-color: var(--color-orange);
  background: rgba(249, 115, 22, 0.15);
  color: var(--color-mustard);
  font-weight: 600;
}

.sidebar-footer {
  padding: var(--spacing-md);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.theme-toggle {
  width: 100%;
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}
</style>
