<template>
  <div class="admin-container">
    <div class="page-header">
      <h2>Painel de Administração</h2>
      <p>Gerencie categorias, projetos e postagens do blog em tempo real.</p>
    </div>

    <!-- Abas do Admin -->
    <div class="tabs-header">
      <button :class="['tab-btn', { active: activeTab === 'projects' }]" @click="activeTab = 'projects'">
        Projetos ({{ projects.length }})
      </button>
      <button :class="['tab-btn', { active: activeTab === 'blog' }]" @click="activeTab = 'blog'">
        Blog ({{ posts.length }})
      </button>
      <button :class="['tab-btn', { active: activeTab === 'categories' }]" @click="activeTab = 'categories'">
        Categorias ({{ categories.length }})
      </button>
    </div>

    <!-- Aba de Categorias -->
    <section v-if="activeTab === 'categories'" class="admin-section">
      <div class="form-card">
        <h3>Nova Categoria</h3>
        <form @submit.prevent="saveCategory" class="admin-form">
          <div class="form-group">
            <label>Nome da Categoria</label>
            <input v-model="newCategory.name" type="text" placeholder="Ex: Cloud & DevOps" required />
          </div>
          <div class="form-group color-group">
            <label>Cor do Marcador</label>
            <div class="color-picker-wrapper">
              <input v-model="newCategory.color" type="color" class="color-picker" />
              <input v-model="newCategory.color" type="text" class="color-text" placeholder="#F97316" />
            </div>
          </div>
          <button type="submit" class="btn-submit">Cadastrar Categoria</button>
        </form>
      </div>

      <div class="list-card">
        <h3>Categorias Cadastradas</h3>
        <div class="items-cards-list">
          <div v-for="cat in categories" :key="cat.id" class="admin-item-card">
            <div class="item-info">
              <span class="color-indicator" :style="{ backgroundColor: cat.color }"></span>
              <strong>{{ cat.name }}</strong>
              <small>({{ cat.id }})</small>
            </div>
            <button class="btn-delete" @click="deleteCategory(cat.id)">Excluir</button>
          </div>
        </div>
      </div>
    </section>

    <!-- Aba de Projetos -->
    <section v-if="activeTab === 'projects'" class="admin-section">
      <div class="form-card">
        <h3>Novo Projeto</h3>
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
            <label>Descrição</label>
            <textarea v-model="newProject.description" rows="3" placeholder="Resumo técnico..." required></textarea>
          </div>
          <div class="form-group">
            <label>Tags (separadas por vírgula)</label>
            <input v-model="projectTagsInput" type="text" placeholder="Rust, 2D, Engine" />
          </div>
          <div class="form-group">
            <label>Link / Repositório</label>
            <input v-model="newProject.link" type="text" placeholder="https://github.com/..." />
          </div>
          <button type="submit" class="btn-submit">Cadastrar Projeto</button>
        </form>
      </div>

      <div class="list-card">
        <h3>Projetos Cadastrados</h3>
        <div class="items-cards-list">
          <div v-for="proj in projects" :key="proj.id" class="admin-item-card">
            <div class="item-info">
              <strong>{{ proj.title }}</strong>
              <p>{{ proj.description }}</p>
            </div>
            <button class="btn-delete" @click="deleteProject(proj.id)">Excluir</button>
          </div>
        </div>
      </div>
    </section>

    <!-- Aba de Blog -->
    <section v-if="activeTab === 'blog'" class="admin-section">
      <div class="form-card">
        <h3>Nova Postagem</h3>
        <form @submit.prevent="saveBlogPost" class="admin-form">
          <div class="form-group">
            <label>Título do Artigo</label>
            <input v-model="newPost.title" type="text" placeholder="Ex: Deploy Contínuo com Rust" required />
          </div>
          <div class="form-group">
            <label>Categoria</label>
            <select v-model="newPost.category_id" required>
              <option value="" disabled>Selecione uma categoria</option>
              <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.name }}</option>
            </select>
          </div>
          <div class="form-group">
            <label>Resumo</label>
            <textarea v-model="newPost.summary" rows="3" placeholder="Resumo do artigo..." required></textarea>
          </div>
          <div class="form-group">
            <label>Tempo de Leitura</label>
            <input v-model="newPost.read_time" type="text" placeholder="Ex: 5 min" required />
          </div>
          <button type="submit" class="btn-submit">Cadastrar Postagem</button>
        </form>
      </div>

      <div class="list-card">
        <h3>Postagens Cadastradas</h3>
        <div class="items-cards-list">
          <div v-for="post in posts" :key="post.id" class="admin-item-card">
            <div class="item-info">
              <strong>{{ post.title }}</strong>
              <p>{{ post.summary }}</p>
            </div>
            <button class="btn-delete" @click="deleteBlogPost(post.id)">Excluir</button>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: 'AdminView',
  props: {
    categories: { type: Array, default: () => [] },
    projects: { type: Array, default: () => [] },
    posts: { type: Array, default: () => [] }
  },
  data() {
    return {
      activeTab: 'projects',
      newCategory: { name: '', color: '#F97316' },
      newProject: { title: '', description: '', category_id: '', link: '' },
      projectTagsInput: '',
      newPost: { title: '', summary: '', category_id: '', read_time: '5 min' }
    };
  },
  methods: {
    async saveCategory() {
      const res = await fetch('http://localhost:8080/api/v1/categories', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: '', name: this.newCategory.name, color: this.newCategory.color })
      });
      if (res.ok) {
        this.newCategory = { name: '', color: '#F97316' };
        this.$emit('refresh');
      }
    },
    async deleteCategory(id) {
      const res = await fetch(`http://localhost:8080/api/v1/categories/${id}`, { method: 'DELETE' });
      if (res.ok) this.$emit('refresh');
    },
    async saveProject() {
      const tags = this.projectTagsInput.split(',').map(t => t.trim()).filter(Boolean);
      const res = await fetch('http://localhost:8080/api/v1/projects', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: '', ...this.newProject, tags })
      });
      if (res.ok) {
        this.newProject = { title: '', description: '', category_id: '', link: '' };
        this.projectTagsInput = '';
        this.$emit('refresh');
      }
    },
    async deleteProject(id) {
      const res = await fetch(`http://localhost:8080/api/v1/projects/${id}`, { method: 'DELETE' });
      if (res.ok) this.$emit('refresh');
    },
    async saveBlogPost() {
      const today = new Date().toISOString().split('T')[0];
      const res = await fetch('http://localhost:8080/api/v1/blog', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ id: '', ...this.newPost, date: today })
      });
      if (res.ok) {
        this.newPost = { title: '', summary: '', category_id: '', read_time: '5 min' };
        this.$emit('refresh');
      }
    },
    async deleteBlogPost(id) {
      const res = await fetch(`http://localhost:8080/api/v1/blog/${id}`, { method: 'DELETE' });
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

.tabs-header {
  display: flex;
  gap: var(--spacing-sm);
  border-bottom: 1px solid var(--border-color);
  padding-bottom: var(--spacing-xs);
}

.tab-btn {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  padding: 8px 16px;
  font-weight: 600;
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: all 0.2s;
}

.tab-btn.active {
  color: var(--color-orange);
  border-bottom-color: var(--color-orange);
}

.admin-section {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-xl);
}

@media (max-width: 900px) {
  .admin-section {
    grid-template-columns: 1fr;
  }
}

.form-card, .list-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
}

.admin-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  margin-top: var(--spacing-md);
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
  padding: 8px 12px;
  border-radius: var(--radius-sm);
  outline: none;
}

.color-picker-wrapper {
  display: flex;
  gap: var(--spacing-sm);
  align-items: center;
}

.color-picker {
  width: 40px;
  height: 36px;
  padding: 0;
  border: none;
  cursor: pointer;
}

.btn-submit {
  background: var(--color-orange);
  color: #fff;
  border: none;
  padding: 10px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  cursor: pointer;
  margin-top: var(--spacing-sm);
}

.items-cards-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-top: var(--spacing-md);
}

.admin-item-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  padding: var(--spacing-md);
  border-radius: var(--radius-sm);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.color-indicator {
  display: inline-block;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  margin-right: var(--spacing-xs);
}

.btn-delete {
  background: rgba(239, 68, 68, 0.15);
  color: var(--color-error);
  border: 1px solid var(--color-error);
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  cursor: pointer;
}
</style>
