<template>
  <aside :class="['sidebar', { collapsed: isCollapsed }]">
    <div class="sidebar-header">
      <div class="logo-box">
        <span class="logo-badge">LC</span>
        <span v-if="!isCollapsed" class="logo-text">Leandro Dev</span>
      </div>
      <button class="toggle-btn" @click="toggleSidebar" :title="isCollapsed ? 'Expandir' : 'Recolher'">
        <svg class="icon-svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path v-if="isCollapsed" d="M13 5l7 7-7 7M5 5l7 7-7 7" />
          <path v-else d="M11 19l-7-7 7-7M19 19l-7-7 7-7" />
        </svg>
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
            <!-- Ícones SVG Nativos e Sólidos -->
            <span class="icon-box">
              <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
                <path :d="item.svgPath" />
              </svg>
            </span>
            <span v-if="!isCollapsed" class="label">{{ item.label }}</span>
          </a>
        </li>
      </ul>
    </nav>

    <div class="sidebar-footer">
      <button class="theme-toggle" @click="cycleTheme" :title="'Tema atual: ' + currentTheme">
        <span class="icon-box">
          <svg class="nav-icon" viewBox="0 0 24 24" fill="currentColor">
            <path v-if="currentTheme === 'light'" d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1z" />
            <path v-else-if="currentTheme === 'dark'" d="M12 3c-4.97 0-9 4.03-9 9 0 2.12.74 4.07 1.97 5.61.43.54 1.23.58 1.71.1.48-.48.44-1.28-.1-1.71C4.65 14.89 4 13.52 4 12c0-4.41 3.59-8 8-8 1.52 0 2.89.65 4 1.58.43.54 1.23.58 1.71.1.48-.48.44-1.28-.1-1.71C16.07 3.74 14.12 3 12 3z" />
            <path v-else d="M4 6h16v10H4zM20 18H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2h16c1.1 0 2 .9 2 2v10c0 1.1-.9 2-2 2zm-8 1.5l1.5 1.5h-3l1.5-1.5z" />
          </svg>
        </span>
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
        { 
          path: '/', 
          label: 'Home', 
          svgPath: 'M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z' 
        },
        { 
          path: '/sobre', 
          label: 'Sobre', 
          svgPath: 'M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z' 
        },
        { 
          path: '/projetos', 
          label: 'Projetos', 
          svgPath: 'M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z' 
        },
        { 
          path: '/curriculo', 
          label: 'Currículo', 
          svgPath: 'M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z' 
        },
        { 
          path: '/blog', 
          label: 'Blog', 
          svgPath: 'M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z' 
        },
        { 
          path: '/contato', 
          label: 'Contato', 
          svgPath: 'M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z' 
        },
        { 
          path: '/admin', 
          label: 'Admin', 
          svgPath: 'M19.43 12.98c.04-.32.07-.64.07-.98s-.03-.66-.07-.98l2.11-1.65c.19-.15.24-.42.12-.64l-2-3.46c-.12-.22-.39-.3-.61-.22l-2.49 1c-.52-.4-1.08-.73-1.69-.98l-.38-2.65C14.46 2.18 14.25 2 14 2h-4c-.25 0-.46.18-.49.42l-.38 2.65c-.61.25-1.17.59-1.69.98l-2.49-1c-.23-.09-.49 0-.61.22l-2 3.46c-.13.22-.07.49.12.64l2.11 1.65c-.04.32-.07.65-.07.98s.03.66.07.98l-2.11 1.65c-.19.15-.24.42-.12.64l2 3.46c.12.22.39.3.61.22l2.49-1c.52.4 1.08.73 1.69.98l.38 2.65c.03.24.24.42.49.42h4c.25 0 .46-.18.49-.42l.38-2.65c.61-.25 1.17-.59 1.69-.98l2.49 1c.23.09.49 0 .61-.22l2-3.46c.12-.22.07-.49-.12-.64l-2.11-1.65zM12 15.5c-1.93 0-3.5-1.57-3.5-3.5s1.57-3.5 3.5-3.5 3.5 1.57 3.5 3.5-1.57 3.5-3.5 3.5z' 
        }
      ]
    };
  },
  computed: {
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
  border-bottom: 1px solid var(--border-color);
}

.logo-box {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.logo-badge {
  background: var(--color-orange);
  color: #FFFFFF;
  font-weight: bold;
  font-size: 0.85rem;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
}

.logo-text {
  color: var(--text-sidebar-active);
  font-weight: 600;
  white-space: nowrap;
}

.toggle-btn, .theme-toggle {
  background: transparent;
  border: none;
  color: var(--text-sidebar);
  cursor: pointer;
  padding: 6px;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  transition: background var(--transition-speed), color var(--transition-speed);
}

.toggle-btn:hover, .theme-toggle:hover {
  background: rgba(255, 255, 255, 0.08);
  color: var(--text-sidebar-active);
}

.icon-svg {
  width: 18px;
  height: 18px;
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
  border-left: 3px solid transparent;
  transition: all 0.2s ease;
}

.icon-box {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
}

.nav-icon {
  width: 20px;
  height: 20px;
  fill: currentColor; /* O ícone herda estritamente a cor da fonte */
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.04);
  color: var(--text-sidebar-active);
}

/* Item Ativo com Indicador Laranja Codium */
.nav-item.active {
  border-left-color: var(--color-orange);
  background: rgba(249, 115, 22, 0.08);
  color: var(--color-mustard);
  font-weight: 600;
}

.nav-item.active .nav-icon {
  fill: var(--color-mustard);
}

.sidebar-footer {
  padding: var(--spacing-md);
  border-top: 1px solid var(--border-color);
}

.theme-toggle {
  width: 100%;
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}
</style>
