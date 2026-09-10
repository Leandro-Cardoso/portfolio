export type NavigationItem = {
  label: string
  to: string
}

const navigation: NavigationItem[] = [
  {
    label: "Home",
    to: "/",
  },
  {
    label: "Projetos",
    to: "/projects",
  },
  {
    label: "Blog",
    to: "/#blog",
  },
  {
    label: "Sobre",
    to: "/#about",
  },
  {
    label: "Contato",
    to: "/#contact",
  },
]

export default navigation
