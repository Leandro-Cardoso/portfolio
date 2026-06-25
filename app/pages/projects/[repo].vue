<template>
  <div
    class="mx-auto max-w-4xl px-6 py-24"
  >
    <div v-if="repo">
      <h1
        class="text-5xl font-semibold tracking-tight"
      >
        {{ repo.name }}
      </h1>

      <p
        class="mt-6 text-lg text-neutral-600 dark:text-neutral-400"
      >
        {{ repo.description }}
      </p>

      <div
        class="mt-8 flex flex-wrap gap-4"
      >
        <span
          class="rounded-lg border px-4 py-2"
        >
          ⭐ {{ repo.stargazers_count }}
        </span>

        <span
          class="rounded-lg border px-4 py-2"
        >
          {{ repo.language }}
        </span>
      </div>

      <div
        class="mt-10 flex flex-wrap gap-4"
      >
        <a
          v-if="repo.homepage"
          :href="repo.homepage"
          target="_blank"
          class="rounded-lg border px-5 py-3"
        >
          Abrir Website
        </a>

        <a
          :href="repo.html_url"
          target="_blank"
          class="rounded-lg border px-5 py-3"
        >
          Ver Repositório
        </a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const route = useRoute()

const repo = await useGithubRepo(
  route.params.repo as string
)
</script>
