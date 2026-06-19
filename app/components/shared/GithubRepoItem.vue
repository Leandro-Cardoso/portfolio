<template>
  <div
    class="flex items-start justify-between gap-4 rounded-lg border p-4 transition hover:bg-black/5"
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
