export const useGithubRepo = async (
  repoName: string
) => {
  const username = "Leandro-Cardoso"

  return await $fetch(
    `https://api.github.com/repos/${username}/${repoName}`
  )
}
