<template>
  <div class="mx-auto max-w-7xl px-4 py-4">
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
      <ProjectsFeaturedProjects />

      <!-- PROJETOS RECENTES -->
      <ProjectsProjectList />

    </div>

  </div>
</template>

<script setup lang="ts">
  const route = useRoute()
</script>
