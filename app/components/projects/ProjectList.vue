<template>
  <section class="mx-auto max-w-7xl px-4 py-4">
    <h2
      class="mb-4 text-3xl font-semibold tracking-tight"
    >
      Projetos Recentes
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
  </section>
</template>

<script setup lang="ts">
    const repos = await useGithubRepos()
</script>
