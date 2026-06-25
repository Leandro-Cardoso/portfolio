<template>
  <section
    id="projects"
    class="mx-auto max-w-7xl px-6 py-24 mb-12"
  >
    <div class="mb-12">
      <h2
        class="mb-4 text-3xl font-bold tracking-tight"
      >
        Projetos em Destaque
      </h2>

      <p class="opacity-70">
        Alguns dos projetos que representam melhor meu trabalho.
      </p>
    </div>

    <div
      class="grid gap-6 md:grid-cols-2 xl:grid-cols-3"
    >
      <SharedProjectCard
        v-for="project in featuredProjects"
        :key="project.slug"
        :slug="project.slug"
        :title="project.name"
        :description="project.description"
        :language="project.language"
        :stars="project.stars"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
const projects = await useGithubRepos()

const featuredProjects = [...projects]
  .sort((a, b) => b.stars - a.stars)
  .slice(0, 3)
</script>
