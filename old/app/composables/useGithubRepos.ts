type Repo = {
  slug: string
  name: string
  description: string
  language: string
  stars: number
  repositoryUrl: string
  homepage: string | null
}

let cache: Repo[] | null = null

export const useGithubRepos = async () => {
  if (cache) {
    return cache
  }

  const username = "Leandro-Cardoso"

  const data = await $fetch<any[]>(
    `https://api.github.com/users/${username}/repos?sort=updated&per_page=100`
  )

  cache = data.map((repo) => ({
    slug: repo.name,
    name: repo.name,
    description: repo.description ?? "",
    language: repo.language ?? "Unknown",
    stars: repo.stargazers_count ?? 0,
    repositoryUrl: repo.html_url,
    homepage: repo.homepage,
  }))

  return cache
}
