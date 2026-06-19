<template>
  <section
    id="blog"
    class="mx-auto max-w-7xl px-6 py-20"
  >
    <div class="mb-10">
      <h2
        class="text-3xl font-bold tracking-tight"
      >
        Blog
      </h2>

      <p class="mt-2 opacity-70">
        Últimos artigos e curiosidades sobre tecnologia e desenvolvimento de software.
      </p>
    </div>

    <div
      class="grid gap-4"
    >
      <SharedBlogCard
        v-for="post in posts"
        :key="post.title"
        :title="post.title"
        :description="post.description"
        :date="post.date"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
const posts = [
  {
    title: "Aprendendo Rust",
    description:
      "Primeiros passos criando bibliotecas reutilizáveis.",
    date: "Jun 2026",
  },

  {
    title: "Docker para APIs",
    description:
      "Estratégias para ambientes consistentes.",
    date: "Jun 2026",
  },

  {
    title: "Arquitetura de Integrações",
    description:
      "Boas práticas em sistemas corporativos.",
    date: "Mai 2026",
  },
]
</script>
