export const useGithubReadme = async (repoName: string) => {
  const username = "Leandro-Cardoso"

  const branches = ["main", "master"]

  for (const branch of branches) {
    try {
      const markdown = await $fetch<string>(
        `https://raw.githubusercontent.com/${username}/${repoName}/${branch}/README.md`
      )

      if (markdown) return markdown
    } catch {}
  }

  return null
}
