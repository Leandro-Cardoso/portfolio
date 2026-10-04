<template>
  <div class="blog-container">
    <div class="page-header">
      <h2>Blog & Artigos Técnicos</h2>
      <p>Artigos sobre Engenharia de Software, Rust, Arquitetura e Cloud.</p>
    </div>

    <CategoryFilter
      :categories="categories"
      :selectedCategory="selectedCategory"
      @change="filterCategory"
    />

    <div v-if="filteredPosts.length" class="cards-grid">
      <ItemCard
        v-for="post in filteredPosts"
        :key="post.id"
        :title="post.title"
        :description="post.summary"
        :category="getCategory(post.category_id)"
        :extraInfo="post.date + ' • ' + post.read_time"
      />
    </div>

    <div v-else class="empty-state">
      <p>Nenhum artigo encontrado para esta categoria.</p>
    </div>
  </div>
</template>

<script>
import CategoryFilter from '../components/CategoryFilter.vue';
import ItemCard from '../components/ItemCard.vue';

export default {
  name: 'BlogView',
  components: { CategoryFilter, ItemCard },
  props: {
    posts: { type: Array, default: () => [] },
    categories: { type: Array, default: () => [] }
  },
  data() {
    return {
      selectedCategory: null
    };
  },
  computed: {
    filteredPosts() {
      if (!this.selectedCategory) return this.posts;
      return this.posts.filter(p => p.category_id === this.selectedCategory);
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
