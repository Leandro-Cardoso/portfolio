<template>
  <div class="md:hidden">
    <!-- Botão -->
    <button
      class="transition-colors duration-200 hover:text-neutral-900 dark:hover:text-white"
      aria-label="Abrir Menu"
      @click="isOpen = !isOpen"
    >
      <span
        class="text-2xl"
      >
        {{ isOpen ? "✕" : "☰" }}
      </span>
    </button>

    <Teleport to="body">
        <!-- Overlay -->
        <Transition
        enter-active-class="transition-opacity duration-200"
        leave-active-class="transition-opacity duration-200"
        enter-from-class="opacity-0"
        leave-to-class="opacity-0"
        >
            <div
                v-if="isOpen"
                class="fixed inset-0 z-40 bg-black/40"
                @click="isOpen = false"
            />
        </Transition>

        <!-- Painel -->
        <Transition
        enter-active-class="transition-transform duration-500 ease-out"
        leave-active-class="transition-transform duration-500 ease-in"
        enter-from-class="-translate-y-full"
        enter-to-class="translate-y-0"
        leave-from-class="translate-y-0"
        leave-to-class="-translate-y-full"
        >
            <aside
                v-if="isOpen"
                class="fixed top-16 right-0 z-50 flex w-full flex-col items-center border-b border-neutral-200 bg-white p-6 dark:border-neutral-800 dark:bg-neutral-950"
            >
                <nav class="flex flex-col gap-6 text-lg items-center">
                    <NuxtLink
                        v-for="item in navigation"
                        :key="item.to"
                        :to="item.to"
                        @click="isOpen = false"
                    >
                        {{ item.label }}
                    </NuxtLink>
                </nav>
            </aside>
        </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
    import navigation from "~/config/navigation"
    const isOpen = ref(false)
    watch(isOpen, (open) => {
        document.body.style.overflow = open ? "hidden" : ""
    })
    onUnmounted(() => {
        document.body.style.overflow = ""
    })
</script>
