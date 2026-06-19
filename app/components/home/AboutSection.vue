<template>
  <section
    id="about"
    class="mx-auto max-w-7xl px-6 py-20"
  >
    <div class="max-w-4xl">
      <h2
        class="mb-8 text-3xl font-bold tracking-tight"
      >
        Sobre Mim
      </h2>

      <p
        class="mb-6 text-lg leading-relaxed opacity-80"
      >
        Sou engenheiro de software e desenvolvedor com foco em testes unitários, arquitetura modular, padrões de projetos, escalabilidade, seguraça e alta performance.
      </p>

      <p
        class="mb-6 text-lg leading-relaxed opacity-80"
      >
        Atualmente trabalho com integrações empresariais,
        APIs REST, Docker, bancos de dados e desenvolvimento
        de soluções para automação de processos e analise de dados.
      </p>

      <p
        class="mb-10 text-lg leading-relaxed opacity-80"
      >
        Tenho especial interesse em engenharia de sofware, desenvolvimento backend, ciências de dados e IA.
      </p>

      <div
        class="flex flex-wrap gap-3"
      >
        <span
          v-for="tech in technologies"
          :key="tech"
          class="rounded-lg border px-4 py-2 text-sm"
        >
          {{ tech }}
        </span>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
const technologies = [
  "Rust",
  "C++",
  "C",
  "Python",
  "Django",
  "Docker",
  "Linux",
  "SQL",
  "PostgreSQL",
  "Git",
  "REST API",
  "Cloud",
  "JavaScript",
  "TypeScript",
  "HTML",
  "CSS",
  "Power BI"
]
</script>
