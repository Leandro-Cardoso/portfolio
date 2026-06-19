type Repo = {
  name: string
  description: string
  language: string
  stars: number
  url: string
}

let cache: Repo[] | null = null

export const useGithubRepos = async () => {
  if (cache) return cache

  const username = "Leandro-Cardoso"

  const data = await $fetch<any[]>(
    `https://api.github.com/users/${username}/repos?sort=updated&per_page=10`
  )

  cache = data.map((repo) => ({
    name: repo.name,
    description: repo.description ?? "",
    language: repo.language ?? "Unknown",
    stars: repo.stargazers_count ?? 0,
    url: repo.html_url,
  }))

  return cache
}
