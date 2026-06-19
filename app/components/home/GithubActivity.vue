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
        href="https://github.com"
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
const repos = [
  {
    name: "logger-rs",
    description: "Biblioteca de logging em Rust",
    language: "Rust",
    stars: 12,
    url: "#",
  },
  {
    name: "sienge-integration",
    description: "Integração com sistemas corporativos",
    language: "Rust",
    stars: 8,
    url: "#",
  },
  {
    name: "game-engine-2d",
    description: "Engine 2D em C++ com SDL2",
    language: "C++",
    stars: 5,
    url: "#",
  },
]
</script>
