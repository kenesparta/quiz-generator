export interface MenuDataItemProps {
  text: string
  link?: string
  submenu?: { text: string; link: string }[]
}

export const MenuData: MenuDataItemProps[] = [
  {
    text: "🏠",
    link: "/",
  },
  {
    text: "👨🏻‍💼 Postulantes",
    link: "#",
    submenu: [
      { text: "Listar todos", link: "/dashboard/applicant" },
    ]
  },
  {
    text: "📝 Evaluaciones",
    link: "#",
    submenu: [
      { text: "Listar todas", link: "/dashboard/evaluation" },
    ]
  },
]

