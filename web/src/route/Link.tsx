import { EVENTS } from "./Consts.ts";
import React from "react";

function navigate(href: string) {
  window.history.pushState({}, '', href)
  const navigateEvent = new Event(EVENTS.NAVIGATION_EVENT)
  window.dispatchEvent(navigateEvent)
}

interface LinkProps {
  target: string
  to: string
  children: string
  props?: object
}

const Link: React.FC<LinkProps> = ({ target, to, children, ...props }) => {
  const handleClick = (event: React.MouseEvent<HTMLAnchorElement, MouseEvent>) => {
    const isMainEvent = event.button !== 0
    const isModifyEvent = event.metaKey || event.altKey || event.ctrlKey || event.shiftKey
    const isManageableEvent = target === undefined || target === '_self'

    if (isMainEvent && isManageableEvent && !isModifyEvent) {
      event.preventDefault()
      navigate(to)
    }
  }

  return (
    <>
      <a className="border-2 p-2 bg-blue-800 text-white" onClick={handleClick} href={to} target={target} {...props}>{children}</a>
    </>
  )
}

export { Link }