import { House, Palette, Settings } from "lucide-react"

export const Siderbar = () => {
  return (
    <nav className="sidebar" role="navigation" aria-label="Main navigation">
      <div className="sidebar__scroll">
        <div className="sidebar__items">
          <button className="sidebar__item" aria-label="Home" title="Home">
              <span><House /></span>
              <span className="sr-only">Home</span>
          </button>
          <button className="sidebar__item" aria-label="Home" title="Home">
              <span><Palette /></span>
              <span className="sr-only">Theme</span>
          </button>
          <button className="sidebar__item" aria-label="Home" title="Home">
              <span><Settings /></span>
              <span className="sr-only">Settings</span>
          </button>
        </div>
      </div>
    </nav>
  )
}
