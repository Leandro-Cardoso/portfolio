<template>
  <article
    class="rounded-lg border border-neutral-200/60 bg-white p-4 transition
    hover:-translate-y-1 hover:border-neutral-300
    dark:border-neutral-800 dark:bg-neutral-900
    dark:hover:border-neutral-700"
  >
    <h3 class="mb-2 font-semibold">
      {{ title }}
    </h3>

    <p class="mb-3 text-sm opacity-70">
      {{ description }}
    </p>

    <div class="text-xs opacity-60">
      {{ date }}
    </div>
  </article>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  description: string
  date: string
}>()
</script>
