export const useGithubRepo = async (repoName: string) => {
  const username = "Leandro-Cardoso"

  try {
    return await $fetch(
      `https://api.github.com/repos/${username}/${repoName}`
    )
  } catch (err) {
    console.error("Erro GitHub repo:", err)
    return null
  }
}
