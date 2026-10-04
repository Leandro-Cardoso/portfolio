<template>
  <div class="category-filter">
    <button
      :class="['filter-chip', { active: selectedCategory === null }]"
      @click="selectCategory(null)"
    >
      Todas
    </button>
    <button
      v-for="cat in categories"
      :key="cat.id"
      :class="['filter-chip', { active: selectedCategory === cat.id }]"
      :style="getChipStyle(cat)"
      @click="selectCategory(cat.id)"
    >
      {{ cat.name }}
    </button>
  </div>
</template>

<script>
export default {
  name: 'CategoryFilter',
  props: {
    categories: { type: Array, required: true },
    selectedCategory: { type: String, default: null }
  },
  methods: {
    selectCategory(catId) {
      this.$emit('change', catId);
    },
    getChipStyle(cat) {
      if (this.selectedCategory === cat.id) {
        return {
          backgroundColor: cat.color,
          borderColor: cat.color,
          color: '#ffffff'
        };
      }
      return {
        borderColor: cat.color,
        color: cat.color
      };
    }
  }
};
</script>

<style scoped>
.category-filter {
  display: flex;
  gap: var(--spacing-sm);
  flex-wrap: wrap;
  margin-bottom: var(--spacing-lg);
}

.filter-chip {
  padding: 6px 14px;
  border-radius: 20px;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-primary);
  font-size: 0.85rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.filter-chip:hover {
  opacity: 0.85;
}

.filter-chip.active {
  background: var(--color-orange);
  border-color: var(--color-orange);
  color: #fff;
}
</style>
