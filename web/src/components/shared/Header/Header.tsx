"use client"
import Link from "next/link"
import Image from "next/image"
import { UserProfileMenu } from "@shared/Header/UserProfileMenu"
import { Menu } from "@shared/Header/Menu";

const MenuImage = () => {
  return (
    <Link href="/" className="flex items-center">
      <Image
        width={40}
        height={30}
        src="/img/logo.png"
        alt="Dashboard Logo"
        quality={90}
        className="mr-2"
      />
      <span className="font-semibold text-lg hidden sm:block">Dashboard</span>
    </Link>
  )
}

export const Header = () => {
  return (
    <header className="bg-blue-600 text-white shadow-md">
      <nav className="grid grid-cols-12 gap-4 items-center px-6 py-3 max-w-7xl mx-auto">
        <div className="col-span-3">
          <MenuImage/>
        </div>

        <div className="col-span-6">
          <Menu/>
        </div>

        <div className="col-span-3 flex justify-end">
          <UserProfileMenu userName="John Doe"/>
        </div>
      </nav>
    </header>
  )
}