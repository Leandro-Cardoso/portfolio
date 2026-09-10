<template>
  <header class="mb-4 sticky top-0 z-50 border-b border-neutral-200/60 bg-white/80 backdrop-blur dark:border-neutral-800 dark:bg-neutral-950/80">
    <div
      class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4"
    >

      <a
        href="#"
        class="text-lg font-semibold tracking-tight transition-colors duration-200 hover:text-neutral-600 dark:hover:text-neutral-300"
      >
        Leandro Cardoso DEV
      </a>

      <nav class="hidden items-center gap-8 text-sm md:flex">
        <NuxtLink
          v-for="item in navigation"
          :key="item.to"
          :to="item.to"
          class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white"
        >
          {{ item.label }}
        </NuxtLink>
      </nav>

      <LayoutMobileMenu />

    </div>
  </header>
</template>

<script setup lang="ts">
  import navigation from "~/config/navigation"
</script>
