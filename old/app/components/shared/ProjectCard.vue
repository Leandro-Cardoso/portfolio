<template>
  <NuxtLink
    :to="`/projects/${slug}`"
    class="block h-full"
  >
    <article
      class="h-full rounded-xl border border-neutral-200/60 bg-white p-4 transition-all duration-200
      hover:-translate-y-1 hover:border-neutral-300 hover:shadow-sm
      dark:border-neutral-800 dark:bg-neutral-900 dark:hover:border-neutral-700"
    >
      <div class="flex h-full flex-col">

        <div>
          <h3 class="mb-4 text-xl font-semibold">
            {{ title }}
          </h3>

          <p
            class="mb-4 line-clamp-3 text-neutral-600 dark:text-neutral-400"
          >
            {{ description }}
          </p>
        </div>

        <div
          class="mt-auto flex items-center justify-between"
        >
          <span
            class="shrink-0 rounded-md border border-neutral-200 px-2 py-1 text-xs
            dark:border-neutral-700"
          >
            {{ language }}
          </span>

          <span
            v-if="stars !== undefined"
            class="text-sm text-neutral-500"
          >
            ⭐ {{ stars }}
          </span>
        </div>

      </div>
    </article>
  </NuxtLink>
</template>

<script setup lang="ts">
  defineProps<{
    slug: string
    title: string
    description: string
    language: string
    stars?: number
  }>()
</script>
