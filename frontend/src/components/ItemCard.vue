<template>
  <div class="item-card">
    <div class="card-header">
      <h3>{{ title }}</h3>
      <span
        v-if="category"
        class="category-badge"
        :style="{ backgroundColor: category.color }"
      >
        {{ category.name }}
      </span>
    </div>
    <p class="card-description">{{ description }}</p>
    
    <div v-if="tags && tags.length" class="tags-container">
      <span v-for="tag in tags" :key="tag" class="tag">{{ tag }}</span>
    </div>

    <div class="card-footer">
      <span v-if="extraInfo" class="extra-info">{{ extraInfo }}</span>
      <a v-if="link" :href="link" target="_blank" class="card-link">Ver Detalhes ➔</a>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ItemCard',
  props: {
    title: { type: String, required: true },
    description: { type: String, required: true },
    category: { type: Object, default: null },
    tags: { type: Array, default: () => [] },
    link: { type: String, default: '' },
    extraInfo: { type: String, default: '' }
  }
};
</script>

<style scoped>
.item-card {
  background-color: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  padding: var(--spacing-lg);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  gap: var(--spacing-md);
  box-shadow: var(--shadow-sm);
  transition: transform 0.2s, box-shadow 0.2s;
}

.item-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--spacing-sm);
}

.card-header h3 {
  font-size: 1.1rem;
  color: var(--text-primary);
}

.category-badge {
  padding: 3px 8px;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 700;
  color: #fff;
  white-space: nowrap;
}

.card-description {
  font-size: 0.9rem;
  color: var(--text-secondary);
  line-height: 1.5;
}

.tags-container {
  display: flex;
  gap: var(--spacing-xs);
  flex-wrap: wrap;
}

.tag {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  padding: 2px 8px;
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--spacing-sm);
  padding-top: var(--spacing-sm);
  border-top: 1px solid var(--border-color);
}

.extra-info {
  font-size: 0.8rem;
  color: var(--text-secondary);
}

.card-link {
  color: var(--color-orange);
  text-decoration: none;
  font-size: 0.85rem;
  font-weight: 600;
}

.card-link:hover {
  text-decoration: underline;
}
</style>
