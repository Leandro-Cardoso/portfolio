<template>
  <section
    id="github"
    class="mx-auto max-w-7xl px-6 py-20"
  >
    <div class="mb-10 flex items-end justify-between">
      <div>
        <h2 class="text-3xl font-bold tracking-tight">
          Projetos Recentes
        </h2>

        <p class="mt-2 opacity-70">
          Repositórios recentes e atualizações.
        </p>
      </div>

      <a
        href="https://github.com/Leandro-Cardoso?tab=repositories"
        target="_blank"
        class="text-sm font-medium hover:underline"
      >
        Ver todos →
      </a>
    </div>

    <div class="space-y-4">
      <SharedGithubRepoItem
        v-for="repo in repos"
        :key="repo.name"
        v-bind="repo"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
const repos = await useGithubRepos()
</script>
