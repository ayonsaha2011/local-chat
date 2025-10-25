import { useAppStore } from "../store";
import { Users, Settings, Moon, Sun, Menu } from "lucide-react";
import PeerList from "./PeerList";

export default function Sidebar() {
  const { darkMode, toggleDarkMode, sidebarCollapsed, toggleSidebar } =
    useAppStore();

  return (
    <div
      className={`${
        sidebarCollapsed ? "w-20" : "w-80"
      } bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col transition-all duration-300`}
    >
      {/* Header */}
      <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
        <button
          onClick={toggleSidebar}
          className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
        >
          <Menu className="w-5 h-5 text-gray-600 dark:text-gray-400" />
        </button>

        {!sidebarCollapsed && (
          <div className="flex items-center gap-2">
            <button
              onClick={toggleDarkMode}
              className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
            >
              {darkMode ? (
                <Sun className="w-5 h-5 text-gray-600 dark:text-gray-400" />
              ) : (
                <Moon className="w-5 h-5 text-gray-600 dark:text-gray-400" />
              )}
            </button>

            <button className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg">
              <Settings className="w-5 h-5 text-gray-600 dark:text-gray-400" />
            </button>
          </div>
        )}
      </div>

      {/* Peers Section */}
      {!sidebarCollapsed && (
        <div className="flex-1 overflow-hidden flex flex-col">
          <div className="p-4 flex items-center gap-2 border-b border-gray-200 dark:border-gray-700">
            <Users className="w-5 h-5 text-gray-600 dark:text-gray-400" />
            <h2 className="font-semibold text-gray-900 dark:text-white">
              Contacts
            </h2>
          </div>
          <PeerList />
        </div>
      )}
    </div>
  );
}
