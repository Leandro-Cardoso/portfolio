<template>
  <header class="sticky top-0 z-50 border-b border-neutral-200/60 bg-white/80 backdrop-blur dark:border-neutral-800 dark:bg-neutral-950/80">
    <div
      class="mx-auto flex h-16 max-w-7xl items-center justify-between px-6"
    >
      <a
        href="#"
        class="text-lg font-semibold tracking-tight transition-colors duration-200 hover:text-neutral-600 dark:hover:text-neutral-300"
      >
        Leandro Cardoso DEV
      </a>

      <nav class="hidden items-center gap-8 text-sm md:flex">
        <NuxtLink to="/" class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white">
          Home
        </NuxtLink>

        <NuxtLink to="/projects" class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white">
          Projetos
        </NuxtLink>

        <a href="#blog" class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white">
          Blog
        </a>

        <a href="#about" class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white">
          Sobre
        </a>

        <a href="#contact" class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white">
          Contato
        </a>
      </nav>

      <button
        class="md:hidden transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white"
        aria-label="Abrir Menu"
      >
        ☰
      </button>
      <!-- TODO: MENU SANDUICHE FUNCIONAL -->
    </div>
  </header>
</template>
