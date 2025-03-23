import { JSX, useId, useState } from "react"
import Link from "next/link"
import { NormalLink } from "@shared/Header/NormalLink"
import { MenuArrow } from "@shared/Arrow";
import { MenuData, MenuDataItemProps } from "@shared/Header/MenuData";

interface MenuItemProps {
  text: string
  link?: string
  submenu?: { text: string; link: string }[]
}

const MenuItem = ({ text, link = "", submenu }: MenuItemProps) => {
  const [isOpen, setIsOpen] = useState(false)

  if (!submenu) return <NormalLink text={text} link={link}/>

  return (
    <div className="relative">
      <a
        onClick={() => setIsOpen(!isOpen)}
        className={`
          flex items-center justify-center px-3 py-2 rounded-md transition-colors
          hover:text-black hover:bg-gray-200 
          ${isOpen ? 'bg-gray-200 text-white dark:bg-gray-600' : ''}
          w-auto
        `}
        aria-expanded={isOpen}
        aria-haspopup="true"
      >
        <span className="mr-1">{text}</span>
        <MenuArrow isOpen={isOpen}/>
      </a>

      {isOpen && (
        <div
          className="absolute top-full left-0 mt-1 w-48 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-10"
          role="menu"
        >
          <div className="py-1">
            {submenu.map((item, index) => (
              <Link
                key={index}
                href={item.link}
                className="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700"
                role="menuitem"
              >
                {item.text}
              </Link>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}

export const Menu = () => {
  const baseId = useId()
  return (
    <div className="flex gap-1">
      {
        MenuData.reduce((acc: JSX.Element[], item: MenuDataItemProps, index) => {
          acc.push(
            <MenuItem
              key={`${baseId}-menu-${index}`}
              text={item.text}
              link={item.link}
              submenu={item.submenu}
            />
          )
          return acc
        }, [])
      }
    </div>
  )
}