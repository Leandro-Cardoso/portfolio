<template>
  <section
    id="projects"
    class="mx-auto max-w-7xl px-4 py-4"
  >

    <div class="mb-4">

      <h2 class="mb-4 text-3xl font-bold tracking-tight">
        {{ props.title }}
      </h2>

      <div class="flex gap-4 justify-between">

        <p class="opacity-70">
          Alguns dos projetos que representam melhor meu trabalho.
        </p>

        <NuxtLink
          v-if="props.showMore"
          to="/projects"
          class="text-sm font-medium transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white"
        >
          Ver todos →
        </NuxtLink>

      </div>

    </div>

    <div
      class="grid items-stretch gap-4 md:grid-cols-2 xl:grid-cols-3"
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
  const featuredProjects = await useGithubPinnedRepos()
  const props = withDefaults(
    defineProps<{
      title?: string
      showMore?: boolean
    }>(),
    {
      title: "Projetos em Destaque",
      showMore: false,
    }
  )
</script>
