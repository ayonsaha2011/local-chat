import { useAppStore } from "../store";
import { Circle } from "lucide-react";
import { UserStatus } from "../types";

export default function PeerList() {
  const { peers, activeChatId, setActiveChatId } = useAppStore();

  const getStatusColor = (status: UserStatus) => {
    switch (status) {
      case UserStatus.Online:
        return "text-green-500";
      case UserStatus.Away:
        return "text-yellow-500";
      case UserStatus.Busy:
        return "text-red-500";
      default:
        return "text-gray-400";
    }
  };

  const getInitials = (name: string) => {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  };

  return (
    <div className="flex-1 overflow-y-auto chat-scrollbar">
      {peers.length === 0 ? (
        <div className="p-4 text-center text-gray-500 dark:text-gray-400">
          <p className="text-sm">No peers discovered yet</p>
          <p className="text-xs mt-1">
            Waiting for others to join the network...
          </p>
        </div>
      ) : (
        <div className="divide-y divide-gray-200 dark:divide-gray-700">
          {peers.map((peer) => (
            <button
              key={peer.profile.user_id}
              onClick={() => setActiveChatId(peer.profile.user_id)}
              className={`w-full p-4 flex items-center gap-3 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors ${
                activeChatId === peer.profile.user_id
                  ? "bg-primary-50 dark:bg-primary-900/20"
                  : ""
              }`}
            >
              {/* Avatar */}
              <div className="relative">
                <div className="w-12 h-12 rounded-full bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center text-white font-semibold">
                  {getInitials(peer.profile.display_name)}
                </div>
                <Circle
                  className={`w-3 h-3 absolute bottom-0 right-0 ${getStatusColor(
                    peer.profile.status
                  )} fill-current`}
                />
              </div>

              {/* Info */}
              <div className="flex-1 text-left">
                <h3 className="font-medium text-gray-900 dark:text-white">
                  {peer.profile.display_name}
                </h3>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  @{peer.profile.username}
                </p>
              </div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
