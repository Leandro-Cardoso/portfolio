import { featuredRepos } from "~/config/featuredRepos"

export const useFeaturedRepos = async () => {
  const repos = await useGithubRepos()

  return repos.filter(repo =>
    featuredRepos.includes(repo.name)
  )
}
