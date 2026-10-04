<template>
  <div class="projects-container">
    <div class="page-header">
      <h2>Projetos & Desenvolvimentos</h2>
      <p>Explore as soluções desenvolvidas em Web, Jogos, Bibliotecas e Inteligência Artificial.</p>
    </div>

    <!-- Filtro de Categorias em Chips -->
    <CategoryFilter
      :categories="categories"
      :selectedCategory="selectedCategory"
      @change="filterCategory"
    />

    <!-- Listagem em Cards Menores -->
    <div v-if="filteredProjects.length" class="cards-grid">
      <ItemCard
        v-for="project in filteredProjects"
        :key="project.id"
        :title="project.title"
        :description="project.description"
        :category="getCategory(project.category_id)"
        :tags="project.tags"
        :link="project.link"
      />
    </div>

    <div v-else class="empty-state">
      <p>Nenhum projeto encontrado para esta categoria.</p>
    </div>
  </div>
</template>

<script>
import CategoryFilter from '../components/CategoryFilter.vue';
import ItemCard from '../components/ItemCard.vue';

export default {
  name: 'ProjectsView',
  components: { CategoryFilter, ItemCard },
  props: {
    projects: { type: Array, default: () => [] },
    categories: { type: Array, default: () => [] }
  },
  data() {
    return {
      selectedCategory: null
    };
  },
  computed: {
    filteredProjects() {
      if (!this.selectedCategory) return this.projects;
      return this.projects.filter(p => p.category_id === this.selectedCategory);
    }
  },
  methods: {
    filterCategory(catId) {
      this.selectedCategory = catId;
    },
    getCategory(catId) {
      return this.categories.find(c => c.id === catId);
    }
  }
};
</script>

<style scoped>
.page-header {
  margin-bottom: var(--spacing-lg);
}

.page-header h2 {
  font-size: 1.6rem;
  color: var(--text-primary);
}

.page-header p {
  color: var(--text-secondary);
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: var(--spacing-lg);
}

.empty-state {
  padding: var(--spacing-xl);
  text-align: center;
  color: var(--text-secondary);
  border: 1px dashed var(--border-color);
  border-radius: var(--radius-md);
}
</style>
