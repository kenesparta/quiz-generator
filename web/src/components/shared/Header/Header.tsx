"use client"
import Link from "next/link"
import Image from "next/image"
import { UserProfileMenu } from "@shared/Header/UserProfileMenu"
import { Menu } from "@shared/Header/Menu";

const MenuImage = () => {
  return (
    <Link href="/" className="">
      <Image
        width={40}
        height={30}
        src="/img/logo.png"
        alt="Dashboard Logo"
        quality={90}
        className="mr-2"
      />
    </Link>
  )
}

export const Header = () => {
  return (
    <header className="bg-blue-600 text-white shadow-md">
      <nav className="grid grid-rows-1 grid-cols-8 items-center gap-4 px-6 py-3 max-w-7xl mx-auto">
        <div className="col-span-1 items-start">
          <MenuImage/>
        </div>

        <div className="col-span-6 justify-self-end">
          <Menu/>
        </div>

        <div className="col-span-1 justify-self-end">
          <UserProfileMenu userName="John Doe"/>
        </div>
      </nav>
    </header>
  )
}