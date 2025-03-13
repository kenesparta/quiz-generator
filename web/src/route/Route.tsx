import React from "react";

const NAVIGATION_EVENT = 'pushState'

function navigate(href: string) {
  window.history.pushState({}, '', href)
  const navigateEvent = new Event(NAVIGATION_EVENT)
  window.dispatchEvent(navigateEvent)
}

const HomePage: React.FC = () => {
  return (
    <>
      <h1>Home</h1>
    </>
  )
}

const About: React.FC = () => {
  return (
    <>
      <h1>About</h1>
    </>
  )
}

const Router: React.FC = () => {
  return (
    <>
      <HomePage/>
      <About/>
    </>
  )
}

export { Router }