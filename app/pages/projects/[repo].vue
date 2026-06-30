<template>
  <div class="mx-auto max-w-5xl px-0 py-0">
    <div v-if="repo">
      <h1 class="text-5xl font-semibold tracking-tight">
        {{ repo.name }}
      </h1>

      <p class="mt-4 text-lg text-neutral-600 dark:text-neutral-400">
        {{ repo.description }}
      </p>

      <div class="mt-4 flex justify-between gap-4">
        <span class="rounded-lg border px-4 py-2">
          {{ repo.language }}
        </span>
        <span class="px-4 py-2">
          ⭐ {{ repo.stargazers_count }}
        </span>
      </div>

      <div class="mt-4 flex flex-wrap justify-center gap-4">
        <a
          v-if="repo.homepage"
          :href="repo.homepage"
          target="_blank"
          class="rounded-lg border px-4 py-3"
        >
          Abrir Website
        </a>

        <a
          :href="repo.html_url"
          target="_blank"
          class="rounded-lg border px-4 py-3"
        >
          Ver Repositório
        </a>
      </div>

      <div
        v-if="readmeHtml"
        class="markdown my-8"
        v-html="readmeHtml"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
  import { marked } from "marked"

  const route = useRoute()

  const repo = ref<any>(null)
  const readmeHtml = ref<string | null>(null)

  const load = async (name: string) => {
    repo.value = await useGithubRepo(name)

    const markdown = await useGithubReadme(name)

    const username = "Leandro-Cardoso"

    const fixedMarkdown = markdown.replace(
      /<img\s+([^>]*?)src="([^":]+)"/g,
      (_, attrs, src) => {
        return `<img ${attrs}src="https://raw.githubusercontent.com/${username}/${name}/main/${src}"`
      }
    )

    readmeHtml.value = fixedMarkdown
      ? await marked.parse(fixedMarkdown)
      : null
  }

  await load(route.params.repo as string)

  watch(
    () => route.params.repo,
    async (newRepo) => {
      if (newRepo) {
        await load(newRepo as string)
      }
    }
  )
</script>
