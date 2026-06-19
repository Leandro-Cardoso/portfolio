<template>
  <section
    id="projects"
    class="mx-auto max-w-7xl px-6 py-20"
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
        v-for="project in projects"
        :key="project.title"
        :title="project.title"
        :description="project.description"
        :language="project.language"
        :repository-url="project.repositoryUrl"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
// Projetos pinados no GitHub
const projects = [
  {
    title: "2D Game Engine",
    description:
      "Engine 2D desenvolvida em C++ utilizando SDL2.",
    language: "C++",
    repositoryUrl: "https://github.com/Leandro-Cardoso/2d-game-engine/",
  },

  {
    title: "Portfólio Antigo",
    description:
      "Versão antiga do meu portfolio.",
    language: "JavaSccript",
    repositoryUrl: "https://github.com/Leandro-Cardoso/DEV",
  },

  {
    title: "Emprego Seguro API",
    description:
      "Trabalho sobre o projeto do app Emprego Seguro. Trabalho realizado para a disciplina de Práticas Extensionistas Integradoras IV, 4º período, turma A, curso de Engenharia de Software da Universidade de Vassouras (Univassouras), ministrado pelo professor Dr. Diego Ramos Inácio.",
    language: "Python",
    repositoryUrl: "https://github.com/Leandro-Cardoso/Emprego-Seguro-api",
  },

  {
    title: "Minefield",
    description:
      "Jogo Campo Minado desenvolvido em Java para rodar no terminal.",
    language: "Java",
    repositoryUrl: "https://github.com/Leandro-Cardoso/GAME-Minefield-Java",
  },

  {
    title: "Sea Battle",
    description:
      "Jogo Batalha Naval desenvolvido em Python para rodar no terminal.",
    language: "Python",
    repositoryUrl: "https://github.com/Leandro-Cardoso/GAME-Sea-Battle-CMD",
  },
  
  {
    title: "Tic-Tac-Toe",
    description:
      "Tic Tac Toe by Código Fonte TV.",
    language: "JavaScript",
    repositoryUrl: "https://github.com/Leandro-Cardoso/tic-tac-toe",
  },
]
</script>
