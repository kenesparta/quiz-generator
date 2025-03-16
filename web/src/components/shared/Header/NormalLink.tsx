import Link from "next/link"

interface NormalLinkProps {
  text: string
  link: string
}

export const NormalLink = ({ text, link }: NormalLinkProps) => {
  return (
    <Link
      href={link}
      className={`
          flex items-center justify-center px-3 py-2 rounded-md transition-colors
          hover:text-black hover:bg-gray-200 
          dark:text-gray-200 dark:hover:bg-gray-700
          w-auto
        `}
    >
      <span>{text}</span>
    </Link>
  )
}
