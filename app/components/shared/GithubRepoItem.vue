<template>
  <div
    class="flex items-start justify-between gap-4 rounded-lg border
    border-neutral-200/60 bg-white p-4 transition
    hover:bg-neutral-50 dark:border-neutral-800 dark:bg-neutral-900
    dark:hover:bg-neutral-800/40"
  >
    <div class="min-w-0">
      <h3 class="font-medium truncate">
        {{ name }}
      </h3>

      <p class="text-sm opacity-70 line-clamp-2">
        {{ description }}
      </p>

      <div class="mt-2 flex gap-3 text-xs opacity-60">
        <span>{{ language }}</span>
        <span v-if="stars">⭐ {{ stars }}</span>
      </div>
    </div>

    <a
      :href="url"
      target="_blank"
      class="text-sm font-medium hover:underline shrink-0"
    >
      GitHub →
    </a>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  name: string
  description?: string
  language?: string
  stars?: number
  url: string
}>()
</script>
