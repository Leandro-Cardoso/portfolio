<template>
  <article
    class="rounded-xl border p-6 transition-all hover:-translate-y-1 hover:shadow-lg"
  >
    <div class="mb-4 flex items-center justify-between">
      <h3 class="text-xl font-semibold">
        {{ title }}
      </h3>

      <span
        class="rounded-md border px-2 py-1 text-xs"
      >
        {{ language }}
      </span>
    </div>

    <p
      class="mb-6 leading-relaxed opacity-80"
    >
      {{ description }}
    </p>

    <div class="flex gap-4">
      <a
        :href="repositoryUrl"
        target="_blank"
        class="text-sm font-medium hover:underline"
      >
        GitHub →
      </a>

      <a
        v-if="projectUrl"
        :href="projectUrl"
        target="_blank"
        class="text-sm font-medium hover:underline"
      >
        Demo →
      </a>
    </div>
  </article>
</template>

<script setup lang="ts">
defineProps<{
  title: string
  description: string
  language: string
  repositoryUrl: string
  projectUrl?: string
}>()
</script>
