import { useState } from "react";
import { useAppStore } from "../store";
import { Send, Paperclip, MessageSquare } from "lucide-react";
import { sendMessage } from "../api";
import { formatDistance } from "date-fns";
import { MessageStatus } from "../types";

export default function ChatWindow() {
  const { activeChatId, peers, messages, userProfile, addMessage } =
    useAppStore();
  const [inputMessage, setInputMessage] = useState("");

  const activePeer = peers.find((p) => p.profile.user_id === activeChatId);
  const chatMessages = messages.filter(
    (m) =>
      (m.sender_id === activeChatId && m.recipient_id === userProfile?.user_id) ||
      (m.sender_id === userProfile?.user_id && m.recipient_id === activeChatId)
  );

  const handleSendMessage = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputMessage.trim() || !activeChatId) return;

    try {
      const message = await sendMessage(activeChatId, inputMessage);
      addMessage(message);
      setInputMessage("");
    } catch (error) {
      console.error("Failed to send message:", error);
    }
  };

  if (!activeChatId || !activePeer) {
    return (
      <div className="flex-1 flex items-center justify-center bg-gray-50 dark:bg-gray-900">
        <div className="text-center">
          <MessageSquare className="w-16 h-16 text-gray-400 mx-auto mb-4" />
          <h3 className="text-xl font-medium text-gray-900 dark:text-white mb-2">
            Select a contact
          </h3>
          <p className="text-gray-500 dark:text-gray-400">
            Choose a contact from the sidebar to start chatting
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col bg-gray-50 dark:bg-gray-900">
      {/* Chat Header */}
      <div className="p-4 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center gap-3">
        <div className="w-10 h-10 rounded-full bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center text-white font-semibold">
          {activePeer.profile.display_name
            .split(" ")
            .map((n) => n[0])
            .join("")
            .toUpperCase()
            .slice(0, 2)}
        </div>
        <div>
          <h2 className="font-semibold text-gray-900 dark:text-white">
            {activePeer.profile.display_name}
          </h2>
          <p className="text-sm text-gray-500 dark:text-gray-400">
            {activePeer.address.ip}
          </p>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto chat-scrollbar p-4 space-y-4">
        {chatMessages.length === 0 ? (
          <div className="text-center text-gray-500 dark:text-gray-400 mt-8">
            <p>No messages yet</p>
            <p className="text-sm mt-1">Send a message to start the conversation</p>
          </div>
        ) : (
          chatMessages.map((message) => {
            const isOwn = message.sender_id === userProfile?.user_id;
            return (
              <div
                key={message.id}
                className={`flex ${isOwn ? "justify-end" : "justify-start"}`}
              >
                <div
                  className={`max-w-md px-4 py-2 rounded-2xl ${
                    isOwn
                      ? "bg-primary-600 text-white"
                      : "bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                  }`}
                >
                  <p className="break-words">{message.content}</p>
                  <div
                    className={`text-xs mt-1 flex items-center gap-1 ${
                      isOwn ? "text-primary-100" : "text-gray-500 dark:text-gray-400"
                    }`}
                  >
                    <span>
                      {formatDistance(new Date(message.timestamp), new Date(), {
                        addSuffix: true,
                      })}
                    </span>
                    {isOwn && (
                      <span className="ml-1">
                        {message.status === MessageStatus.Sent && "✓"}
                        {message.status === MessageStatus.Delivered && "✓✓"}
                        {message.status === MessageStatus.Read && "✓✓"}
                      </span>
                    )}
                  </div>
                </div>
              </div>
            );
          })
        )}
      </div>

      {/* Input */}
      <div className="p-4 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700">
        <form onSubmit={handleSendMessage} className="flex items-center gap-2">
          <button
            type="button"
            className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
          >
            <Paperclip className="w-5 h-5 text-gray-600 dark:text-gray-400" />
          </button>

          <input
            type="text"
            value={inputMessage}
            onChange={(e) => setInputMessage(e.target.value)}
            placeholder="Type a message..."
            className="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
          />

          <button
            type="submit"
            disabled={!inputMessage.trim()}
            className="p-2 bg-primary-600 hover:bg-primary-700 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Send className="w-5 h-5" />
          </button>
        </form>
      </div>
    </div>
  );
}
