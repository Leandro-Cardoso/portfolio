type Repo = {
  slug: string
  name: string
  description: string
  language: string
  stars: number
  repositoryUrl: string
  homepage: string | null
}

const PINNED_REPOSITORIES = [
  "2d-game-engine",
  "DEV",
  "ToDo",
  "Logger",
  "BRDoc",
  "TaskFlow"
]

let cache: Repo[] | null = null

export const useGithubPinnedRepos = async (): Promise<Repo[]> => {
  if (cache) {
    return cache
  }

  const repos = await useGithubRepos()

  cache = PINNED_REPOSITORIES
    .map((name) => repos.find((repo) => repo.slug === name))
    .filter((repo): repo is Repo => repo !== undefined)

  return cache
}
