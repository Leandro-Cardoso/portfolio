<template>
  <div class="mx-auto max-w-7xl px-6 py-24">
    <div class="mb-12">

      <h1 class="text-5xl font-semibold tracking-tight">
        Projetos
      </h1>

      <p class="mt-4 text-neutral-600 dark:text-neutral-400">
        Todos os meus projetos públicos.
      </p>

    </div>

    <!-- IMPORTANTE: outlet da rota filha -->
    <NuxtPage />

    <!-- FALLBACK - SE NÃO /projects/:repo -->
    <div v-if="!route.params.repo">

      <!-- PROJETOS EM DESTAQUE -->
      <HomeFeaturedProjects />

      <!-- PROJETOS RECENTES -->
      <h2 class="mb-4 text-3xl font-semibold tracking-tight">
        Recentes
      </h2>
      <div class="grid gap-4">
        <SharedProjectCard
          v-for="repo in repos"
          :key="repo.slug"
          :slug="repo.slug"
          :title="repo.name"
          :description="repo.description"
          :language="repo.language"
          :stars="repo.stars"
        />
      </div>

    </div>

  </div>
</template>

<script setup lang="ts">
  const route = useRoute()
  const repos = await useGithubRepos()
</script>
