import React, { useEffect, useState } from "react";
import { EVENTS } from "./Consts.ts";
import { Link } from "./Link.tsx";

const HomePage: React.FC = () => {
  return (
    <>
      <h1 className="text-6xl">Home</h1>
      <p className="text-xl mb-4">Welcome to my website!!!</p>
      <Link target={"_self_"} to={"/about"}>Go to About</Link>
    </>
  )
}

const About: React.FC = () => {
  return (
    <>
      <h1 className="text-6xl">About</h1>
      <div className="mb-2"> Hello there!</div>
      <Link target={"_self_"} to={"/"}> Go to home</Link>
    </>
  )
}

const Router: React.FC = () => {
  const [currPath, setCurrPath] = useState<string>(window.location.pathname)
  useEffect(() => {
    const onLocationChange = () => {
      setCurrPath(window.location.pathname)
    }

    window.addEventListener(EVENTS.NAVIGATION_EVENT, onLocationChange)
    window.addEventListener(EVENTS.POPSTATE, onLocationChange)

    return () => {
      window.removeEventListener(EVENTS.NAVIGATION_EVENT, onLocationChange)
      window.addEventListener(EVENTS.POPSTATE, onLocationChange)
    }
  }, [])
  return (
    <>
      {currPath === '/' && <HomePage/>}
      {currPath === '/about' && <About/>}
    </>
  )
}

export { Router }