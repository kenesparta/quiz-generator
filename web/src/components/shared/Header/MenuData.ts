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
    text: "👨🏻‍💼 Postulante",
    link: "#",
    submenu: [
      { text: "Listar", link: "/dashboard/applicant" },
      { text: "Agregar", link: "/dashboard/applicant/new" },
    ]
  },
  {
    text: "📝 Evaluacion",
    link: "#",
    submenu: [
      { text: "Listar", link: "/dashboard/evaluation" },
      { text: "Calificar", link: "/dashboard/evaluation" },
    ]
  },
  {
    text: "📈 Reportes",
    link: "#",
    submenu: [
      { text: "Ver reporte 01", link: "/dashboard/evaluation" },
      { text: "Ver Reporte 02", link: "/dashboard/evaluation" },
    ]
  },
]

