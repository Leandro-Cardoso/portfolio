<template>
  <div
    class="mx-auto max-w-7xl px-6 py-24"
  >
    <div class="mb-12">
      <h1
        class="text-5xl font-semibold tracking-tight"
      >
        Projetos
      </h1>

      <p
        class="mt-4 text-neutral-600 dark:text-neutral-400"
      >
        Todos os repositórios públicos e projetos.
      </p>
    </div>

    <div
      class="grid gap-6 md:grid-cols-2 xl:grid-cols-3"
    >
      <SharedProjectCard
        v-for="repo in repos"
        :key="repo.name"
        :title="repo.name"
        :description="repo.description"
        :language="repo.language"
        :repository-url="repo.url"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
const repos = await useGithubRepos()
</script>
