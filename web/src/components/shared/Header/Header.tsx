import Link from "next/link"

interface MenuItemProps {
  text: string
  link: string
}

const MenuItem = ({ text, link }: MenuItemProps) => {
  return (
    <>
      <Link href={link} className="hover:text-black hover:bg-gray-200 dark:text-gray-200 p-2 rounded-md">
        <li>
          {text}
        </li>
      </Link>
    </>
  )
}

export const Header = () => {
  return (
    <header className="bg-blue-500 text-white dark:bg-blue-900 dark:text-gray-200">
      <nav className="container mx-auto flex items-center justify-between py-4">
        <a href="#" className="text-lg font-bold">LOGO</a>
        <ul className="flex space-x-4">
          <MenuItem link="/" text="🏠"/>
          <MenuItem link="/dashboard/applicant" text="👨🏻‍💼 Postulantes"/>
          <MenuItem link="/dashboard/evaluation" text="📝 Evaluaciones"/>
        </ul>
      </nav>
    </header>
  )
}