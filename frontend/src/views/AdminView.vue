<template>
  <div class="admin-container">
    <!-- TELA DE LOGIN SE NÃO AUTENTICADO -->
    <div v-if="!isAuthenticated" class="login-card">
      <div class="login-header">
        <h2>Painel Administrativo</h2>
        <p>Autenticação necessária para alterar o conteúdo do site.</p>
      </div>
      
      <form @submit.prevent="handleLogin" class="admin-form">
        <div class="form-group">
          <label>Usuário</label>
          <input v-model="loginForm.username" type="text" placeholder="admin" required />
        </div>
        <div class="form-group">
          <label>Senha</label>
          <input v-model="loginForm.password" type="password" placeholder="••••••••" required />
        </div>
        <button type="submit" class="btn-primary">Entrar no Sistema 🔓</button>
        <p v-if="loginError" class="error-msg">{{ loginError }}</p>
      </form>
    </div>

    <!-- PAINEL PROTEGIDO COM ABAS E SUB-ABAS -->
    <div v-else class="admin-panel">
      <div class="admin-top-bar">
        <div>
          <h2>Gestão do Conteúdo do Portfolio</h2>
          <p class="sub-text">Selecione a página e a subseção para editar os dados armazenados no SQLite.</p>
        </div>
        <button class="btn-logout" @click="logout">Sair 🔒</button>
      </div>

      <!-- 1º NÍVEL: ABAS DE PÁGINAS -->
      <nav class="main-tabs-header">
        <button
          v-for="page in pages"
          :key="page.id"
          :class="['main-tab-btn', { active: activeMainTab === page.id }]"
          @click="selectMainTab(page.id)"
        >
          <span class="tab-icon">{{ page.icon }}</span>
          <span>{{ page.label }}</span>
        </button>
      </nav>

      <!-- 2º NÍVEL: SUB-ABAS DA PÁGINA SELECIONADA -->
      <nav class="sub-tabs-header">
        <button
          v-for="sub in currentSubTabs"
          :key="sub.id"
          :class="['sub-tab-btn', { active: activeSubTab === sub.id }]"
          @click="activeSubTab = sub.id"
        >
          {{ sub.label }}
        </button>
      </nav>

      <!-- CONTEÚDO DAS SUB-ABAS -->

      <!-- ==================== PÁGINA: HOME ==================== -->
      <section v-if="activeMainTab === 'home'" class="admin-card margin-top">
        <div v-if="activeSubTab === 'home_bio'">
          <h3>Biografia da Home</h3>
          <form @submit.prevent="saveSiteInfo" class="admin-form">
            <div class="form-group">
              <label>Texto do Hero / Apresentação Inicial</label>
              <textarea v-model="editableInfo.bio" rows="4" required></textarea>
            </div>
            <button type="submit" class="btn-primary">Salvar Biografia da Home</button>
          </form>
        </div>

        <div v-if="activeSubTab === 'home_metrics'">
          <h3>Métricas & Status da Dashboard</h3>
          <p class="info-text">As métricas de projetos e postagens são calculadas dinamicamente com base nos dados do SQLite.</p>
          <div class="metrics-preview">
            <div class="preview-box">Projetos Cadastrados: <strong>{{ projects.length }}</strong></div>
            <div class="preview-box">Artigos Publicados: <strong>{{ posts.length }}</strong></div>
            <div class="preview-box">Categorias Ativas: <strong>{{ categories.length }}</strong></div>
          </div>
        </div>
      </section>

      <!-- ==================== PÁGINA: SOBRE ==================== -->
      <section v-if="activeMainTab === 'sobre'" class="admin-card margin-top">
        <div v-if="activeSubTab === 'sobre_bio'">
          <h3>Informações do Perfil</h3>
          <form @submit.prevent="saveSiteInfo" class="admin-form">
            <div class="form-group">
              <label>Biografia Detalhada</label>
              <textarea v-model="editableInfo.bio" rows="5" required></textarea>
            </div>
            <div class="form-group">
              <label>Localização</label>
              <input v-model="editableInfo.location" type="text" required />
            </div>
            <button type="submit" class="btn-primary">Salvar Dados da Página Sobre</button>
          </form>
        </div>

        <div v-if="activeSubTab === 'sobre_pillars'">
          <h3>Pilares Técnicos & Especialidades</h3>
          <p class="info-text">Os pilares técnicos exibem as suas especialidades em Engenharia de Software, Cloud, Banco de Dados e IA.</p>
        </div>
      </section>

      <!-- ==================== PÁGINA: CURRÍCULO ==================== -->
      <section v-if="activeMainTab === 'curriculo'" class="admin-card margin-top">
        <div v-if="activeSubTab === 'cv_summary'">
          <h3>Resumo do Currículo</h3>
          <form @submit.prevent="saveSiteInfo" class="admin-form">
            <div class="form-group">
              <label>Resumo Profissional para Impressão / PDF</label>
              <textarea v-model="editableInfo.cv_summary" rows="6" required></textarea>
            </div>
            <button type="submit" class="btn-primary">Salvar Resumo do CV</button>
          </form>
        </div>

        <div v-if="activeSubTab === 'cv_skills'">
          <h3>Habilidades do Currículo</h3>
          <p class="info-text">As tags de competências principais são geradas automaticamente a partir das suas especialidades e projetos cadastrados.</p>
        </div>
      </section>

      <!-- ==================== PÁGINA: PROJETOS ==================== -->
      <section v-if="activeMainTab === 'projetos'" class="margin-top">
        <div v-if="activeSubTab === 'proj_manage'" class="admin-two-cols">
          <div class="admin-card">
            <h3>Cadastrar Novo Projeto</h3>
            <form @submit.prevent="saveProject" class="admin-form">
              <div class="form-group">
                <label>Título do Projeto</label>
                <input v-model="newProject.title" type="text" placeholder="Ex: Tiles Gear Engine" required />
              </div>
              <div class="form-group">
                <label>Categoria</label>
                <select v-model="newProject.category_id" required>
                  <option value="" disabled>Selecione uma categoria</option>
                  <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.name }}</option>
                </select>
              </div>
              <div class="form-group">
                <label>Descrição Técnica</label>
                <textarea v-model="newProject.description" rows="3" required></textarea>
              </div>
              <div class="form-group">
                <label>Tags (ex: Rust, 2D, Engine)</label>
                <input v-model="newProject.tags" type="text" required />
              </div>
              <div class="form-group">
                <label>Link / Repositório</label>
                <input v-model="newProject.link" type="text" placeholder="https://github.com/..." required />
              </div>
              <button type="submit" class="btn-primary">Cadastrar Projeto</button>
            </form>
          </div>

          <div class="admin-card">
            <h3>Projetos Cadastrados ({{ projects.length }})</h3>
            <div class="items-list">
              <div v-for="p in projects" :key="p.id" class="item-row">
                <div>
                  <strong>{{ p.title }}</strong>
                  <p class="small-desc">{{ p.description }}</p>
                </div>
                <button class="btn-delete" @click="deleteProject(p.id)">Excluir</button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="activeSubTab === 'proj_categories'" class="admin-two-cols">
          <div class="admin-card">
            <h3>Nova Categoria de Projeto</h3>
            <form @submit.prevent="saveCategory" class="admin-form">
              <div class="form-group">
                <label>Nome da Categoria</label>
                <input v-model="newCategory.name" type="text" placeholder="Ex: Jogos / Engine" required />
              </div>
              <div class="form-group">
                <label>Cor do Marcador</label>
                <div class="color-picker-box">
                  <input v-model="newCategory.color" type="color" class="color-picker" />
                  <input v-model="newCategory.color" type="text" class="color-text" required />
                </div>
              </div>
              <button type="submit" class="btn-primary">Cadastrar Categoria</button>
            </form>
          </div>

          <div class="admin-card">
            <h3>Categorias Existentes</h3>
            <div class="items-list">
              <div v-for="c in categories" :key="c.id" class="item-row">
                <div class="cat-label-box">
                  <span class="color-dot" :style="{ backgroundColor: c.color }"></span>
                  <strong>{{ c.name }}</strong>
                </div>
                <button class="btn-delete" @click="deleteCategory(c.id)">Excluir</button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- ==================== PÁGINA: BLOG ==================== -->
      <section v-if="activeMainTab === 'blog'" class="margin-top">
        <div v-if="activeSubTab === 'blog_manage'" class="admin-two-cols">
          <div class="admin-card">
            <h3>Novo Artigo</h3>
            <form @submit.prevent="saveBlogPost" class="admin-form">
              <div class="form-group">
                <label>Título do Artigo</label>
                <input v-model="newPost.title" type="text" placeholder="Ex: Arquitetura de Game Engine" required />
              </div>
              <div class="form-group">
                <label>Categoria</label>
                <select v-model="newPost.category_id" required>
                  <option value="" disabled>Selecione uma categoria</option>
                  <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.name }}</option>
                </select>
              </div>
              <div class="form-group">
                <label>Resumo / Introdução</label>
                <textarea v-model="newPost.summary" rows="3" required></textarea>
              </div>
              <div class="form-group">
                <label>Tempo de Leitura</label>
                <input v-model="newPost.read_time" type="text" placeholder="5 min" required />
              </div>
              <button type="submit" class="btn-primary">Cadastrar Postagem</button>
            </form>
          </div>

          <div class="admin-card">
            <h3>Artigos Publicados ({{ posts.length }})</h3>
            <div class="items-list">
              <div v-for="b in posts" :key="b.id" class="item-row">
                <div>
                  <strong>{{ b.title }}</strong>
                  <p class="small-desc">{{ b.summary }}</p>
                </div>
                <button class="btn-delete" @click="deleteBlogPost(b.id)">Excluir</button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="activeSubTab === 'blog_categories'" class="admin-two-cols">
          <div class="admin-card">
            <h3>Nova Categoria para o Blog</h3>
            <form @submit.prevent="saveCategory" class="admin-form">
              <div class="form-group">
                <label>Nome da Categoria</label>
                <input v-model="newCategory.name" type="text" placeholder="Ex: Tecnologia" required />
              </div>
              <div class="form-group">
                <label>Cor do Marcador</label>
                <div class="color-picker-box">
                  <input v-model="newCategory.color" type="color" class="color-picker" />
                  <input v-model="newCategory.color" type="text" class="color-text" required />
                </div>
              </div>
              <button type="submit" class="btn-primary">Cadastrar Categoria</button>
            </form>
          </div>

          <div class="admin-card">
            <h3>Categorias Existentes</h3>
            <div class="items-list">
              <div v-for="c in categories" :key="c.id" class="item-row">
                <div class="cat-label-box">
                  <span class="color-dot" :style="{ backgroundColor: c.color }"></span>
                  <strong>{{ c.name }}</strong>
                </div>
                <button class="btn-delete" @click="deleteCategory(c.id)">Excluir</button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- ==================== PÁGINA: CONTATO ==================== -->
      <section v-if="activeMainTab === 'contato'" class="admin-card margin-top">
        <div v-if="activeSubTab === 'contato_channels'">
          <h3>Canais Diretos de Contato</h3>
          <form @submit.prevent="saveSiteInfo" class="admin-form">
            <div class="form-grid">
              <div class="form-group">
                <label>E-mail de Contato</label>
                <input v-model="editableInfo.email" type="email" required />
              </div>
              <div class="form-group">
                <label>GitHub</label>
                <input v-model="editableInfo.github" type="text" required />
              </div>
              <div class="form-group">
                <label>LinkedIn</label>
                <input v-model="editableInfo.linkedin" type="text" required />
              </div>
              <div class="form-group">
                <label>Localização</label>
                <input v-model="editableInfo.location" type="text" required />
              </div>
            </div>
            <button type="submit" class="btn-primary">Salvar Canais de Contato</button>
          </form>
        </div>
      </section>
    </div>
  </div>
</template>

<script>
export default {
  name: 'AdminView',
  props: {
    siteInfo: { type: Object, default: () => ({}) },
    projects: { type: Array, default: () => [] },
    posts: { type: Array, default: () => [] },
    categories: { type: Array, default: () => [] }
  },
  data() {
    return {
      isAuthenticated: !!localStorage.getItem('admin_token'),
      loginForm: { username: 'admin', password: '' },
      loginError: '',
      
      // Estado de Navegação por Abas Principais e Sub-abas
      activeMainTab: 'home',
      activeSubTab: 'home_bio',

      pages: [
        { id: 'home', label: 'Home', icon: '🏠' },
        { id: 'sobre', label: 'Sobre', icon: '👤' },
        { id: 'curriculo', label: 'Currículo', icon: '📄' },
        { id: 'projetos', label: 'Projetos', icon: '🚀' },
        { id: 'blog', label: 'Blog', icon: '📝' },
        { id: 'contato', label: 'Contato', icon: '✉️' }
      ],

      subTabsMap: {
        home: [
          { id: 'home_bio', label: 'Biografia Curta' },
          { id: 'home_metrics', label: 'Métricas & Status' }
        ],
        sobre: [
          { id: 'sobre_bio', label: 'Biografia & Perfil' },
          { id: 'sobre_pillars', label: 'Pilares Técnicos' }
        ],
        curriculo: [
          { id: 'cv_summary', label: 'Resumo Profissional' },
          { id: 'cv_skills', label: 'Habilidades' }
        ],
        projetos: [
          { id: 'proj_manage', label: 'Gerenciar Projetos' },
          { id: 'proj_categories', label: 'Categorias de Projetos' }
        ],
        blog: [
          { id: 'blog_manage', label: 'Gerenciar Postagens' },
          { id: 'blog_categories', label: 'Categorias do Blog' }
        ],
        contato: [
          { id: 'contato_channels', label: 'Canais Diretos' }
        ]
      },

      editableInfo: { ...this.siteInfo },
      newProject: { title: '', description: '', category_id: '', tags: '', link: '' },
      newPost: { title: '', summary: '', category_id: '', read_time: '5 min' },
      newCategory: { name: '', color: '#F97316' }
    };
  },
  computed: {
    currentSubTabs() {
      return this.subTabsMap[this.activeMainTab] || [];
    }
  },
  watch: {
    siteInfo: {
      handler(val) { this.editableInfo = { ...val }; },
      deep: true
    }
  },
  methods: {
    selectMainTab(pageId) {
      this.activeMainTab = pageId;
      const subTabs = this.subTabsMap[pageId];
      if (subTabs && subTabs.length) {
        this.activeSubTab = subTabs[0].id;
      }
    },
    async handleLogin() {
      this.loginError = '';
      try {
        const res = await fetch('http://localhost:8080/api/v1/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(this.loginForm)
        });
        if (res.ok) {
          const data = await res.json();
          localStorage.setItem('admin_token', data.token);
          this.isAuthenticated = true;
        } else {
          this.loginError = 'Usuário ou senha inválidos.';
        }
      } catch {
        this.loginError = 'Falha ao conectar com o backend em Rust.';
      }
    },
    logout() {
      localStorage.removeItem('admin_token');
      this.isAuthenticated = false;
    },
    async saveSiteInfo() {
      const token = localStorage.getItem('admin_token');
      const res = await fetch('http://localhost:8080/api/v1/site-info', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify(this.editableInfo)
      });
      if (res.ok) {
        alert('Informações salvas no banco de dados SQLite com sucesso!');
        this.$emit('refresh');
      }
    },
    async saveProject() {
      const token = localStorage.getItem('admin_token');
      const res = await fetch('http://localhost:8080/api/v1/projects', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ id: '', ...this.newProject })
      });
      if (res.ok) {
        this.newProject = { title: '', description: '', category_id: '', tags: '', link: '' };
        this.$emit('refresh');
      }
    },
    async deleteProject(id) {
      const token = localStorage.getItem('admin_token');
      const res = await fetch(`http://localhost:8080/api/v1/projects/${id}`, {
        method: 'DELETE',
        headers: { 'Authorization': `Bearer ${token}` }
      });
      if (res.ok) this.$emit('refresh');
    },
    async saveBlogPost() {
      const token = localStorage.getItem('admin_token');
      const today = new Date().toISOString().split('T')[0];
      const res = await fetch('http://localhost:8080/api/v1/blog', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ id: '', ...this.newPost, date: today })
      });
      if (res.ok) {
        this.newPost = { title: '', summary: '', category_id: '', read_time: '5 min' };
        this.$emit('refresh');
      }
    },
    async deleteBlogPost(id) {
      const token = localStorage.getItem('admin_token');
      const res = await fetch(`http://localhost:8080/api/v1/blog/${id}`, {
        method: 'DELETE',
        headers: { 'Authorization': `Bearer ${token}` }
      });
      if (res.ok) this.$emit('refresh');
    },
    async saveCategory() {
      const token = localStorage.getItem('admin_token');
      const res = await fetch('http://localhost:8080/api/v1/categories', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({ id: '', name: this.newCategory.name, color: this.newCategory.color })
      });
      if (res.ok) {
        this.newCategory = { name: '', color: '#F97316' };
        this.$emit('refresh');
      }
    },
    async deleteCategory(id) {
      const token = localStorage.getItem('admin_token');
      const res = await fetch(`http://localhost:8080/api/v1/categories/${id}`, {
        method: 'DELETE',
        headers: { 'Authorization': `Bearer ${token}` }
      });
      if (res.ok) this.$emit('refresh');
    }
  }
};
</script>

<style scoped>
.admin-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.login-card {
  max-width: 420px;
  margin: 40px auto;
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  padding: var(--spacing-xl);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
}

.login-header h2 {
  font-size: 1.4rem;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
}

.login-header p {
  color: var(--text-secondary);
  font-size: 0.88rem;
  margin-bottom: var(--spacing-lg);
}

.admin-top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-md) var(--spacing-lg);
}

.admin-top-bar h2 {
  font-size: 1.3rem;
  color: var(--text-primary);
}

.sub-text {
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.btn-logout {
  background: rgba(239, 68, 68, 0.15);
  color: var(--color-error);
  border: 1px solid var(--color-error);
  padding: 6px 14px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-weight: 600;
}

/* --- ESTILOS DE ABAS PRINCIPAIS (1º NÍVEL) --- */
.main-tabs-header {
  display: flex;
  gap: var(--spacing-xs);
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-xs);
  margin-top: var(--spacing-md);
  overflow-x: auto;
}

.main-tab-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  background: transparent;
  border: none;
  color: var(--text-secondary);
  padding: 10px 16px;
  font-weight: 600;
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all 0.2s;
  white-space: nowrap;
}

.main-tab-btn:hover {
  background: var(--bg-primary);
  color: var(--text-primary);
}

.main-tab-btn.active {
  background: var(--color-orange);
  color: #ffffff;
}

/* --- ESTILOS DE SUB-ABAS (2º NÍVEL) --- */
.sub-tabs-header {
  display: flex;
  gap: var(--spacing-sm);
  border-bottom: 1px solid var(--border-color);
  padding-bottom: var(--spacing-xs);
  margin-top: var(--spacing-md);
}

.sub-tab-btn {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  padding: 6px 14px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s;
}

.sub-tab-btn.active {
  color: var(--color-mustard);
  border-bottom-color: var(--color-mustard);
}

.margin-top {
  margin-top: var(--spacing-lg);
}

.admin-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
}

.admin-card h3 {
  font-size: 1.1rem;
  color: var(--text-primary);
  margin-bottom: var(--spacing-lg);
}

.info-text {
  color: var(--text-secondary);
  font-size: 0.9rem;
  margin-bottom: var(--spacing-md);
}

.metrics-preview {
  display: flex;
  gap: var(--spacing-md);
  flex-wrap: wrap;
}

.preview-box {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  padding: var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: 0.9rem;
}

.admin-two-cols {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-xl);
}

@media (max-width: 900px) {
  .admin-two-cols {
    grid-template-columns: 1fr;
  }
}

.admin-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.form-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-md);
}

@media (max-width: 600px) {
  .form-grid {
    grid-template-columns: 1fr;
  }
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-group label {
  font-size: 0.85rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.form-group input, .form-group select, .form-group textarea {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
  padding: 10px 12px;
  border-radius: var(--radius-sm);
  outline: none;
  font-family: inherit;
}

.form-group input:focus, .form-group select:focus, .form-group textarea:focus {
  border-color: var(--color-orange);
}

.color-picker-box {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
}

.color-picker {
  width: 44px;
  height: 38px;
  padding: 0;
  border: none;
  cursor: pointer;
  background: transparent;
}

.btn-primary {
  background: var(--color-orange);
  color: #ffffff;
  border: none;
  padding: 12px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  cursor: pointer;
  margin-top: var(--spacing-sm);
}

.items-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
}

.item-row {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  padding: var(--spacing-md);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.small-desc {
  font-size: 0.82rem;
  color: var(--text-secondary);
  margin-top: 2px;
}

.cat-label-box {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.color-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.btn-delete {
  background: rgba(239, 68, 68, 0.15);
  color: var(--color-error);
  border: 1px solid var(--color-error);
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.error-msg {
  color: var(--color-error);
  font-size: 0.85rem;
  text-align: center;
}
</style>
