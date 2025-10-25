import { useEffect } from "react";
import { useAppStore } from "./store";
import {
  initializeApp,
  getPeers,
  getMessages,
  listenToPeerDiscovered,
  listenToMessageReceived,
} from "./api";
import Sidebar from "./components/Sidebar";
import ChatWindow from "./components/ChatWindow";
import WelcomeScreen from "./components/WelcomeScreen";

function App() {
  const { userProfile, setUserProfile, addPeer, addMessage, darkMode } =
    useAppStore();

  useEffect(() => {
    // Set up event listeners
    const setupListeners = async () => {
      await listenToPeerDiscovered((peer) => {
        console.log("Peer discovered:", peer);
        addPeer(peer);
      });

      await listenToMessageReceived((message) => {
        console.log("Message received:", message);
        addMessage(message);
      });
    };

    setupListeners();
  }, [addPeer, addMessage]);

  useEffect(() => {
    // Apply dark mode
    if (darkMode) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }, [darkMode]);

  const handleInitialize = async (username: string, displayName: string) => {
    try {
      const profile = await initializeApp(username, displayName);
      setUserProfile(profile);
      
      console.log("Profile initialized:", profile);

      // Wait a moment for services to start
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Load initial data
      const peers = await getPeers();
      console.log("Initial peers loaded:", peers);
      peers.forEach((peer) => addPeer(peer));

      const messages = await getMessages();
      messages.forEach((msg) => addMessage(msg));
    } catch (error) {
      console.error("Failed to initialize:", error);
    }
  };

  if (!userProfile) {
    return <WelcomeScreen onInitialize={handleInitialize} />;
  }

  return (
    <div className="flex h-screen bg-gray-50 dark:bg-gray-900">
      <Sidebar />
      <ChatWindow />
    </div>
  );
}

export default App;
