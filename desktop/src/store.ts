import { create } from "zustand";
import { UserProfile, Peer, Message, FileTransfer } from "./types";

interface AppStore {
  // User
  userProfile: UserProfile | null;
  setUserProfile: (profile: UserProfile | null) => void;

  // Peers
  peers: Peer[];
  addPeer: (peer: Peer) => void;
  removePeer: (userId: string) => void;
  updatePeer: (peer: Peer) => void;

  // Messages
  messages: Message[];
  addMessage: (message: Message) => void;
  setMessages: (messages: Message[]) => void;

  // Active chat
  activeChatId: string | null;
  setActiveChatId: (userId: string | null) => void;

  // File transfers
  fileTransfers: FileTransfer[];
  addFileTransfer: (transfer: FileTransfer) => void;
  updateFileTransfer: (transfer: FileTransfer) => void;

  // UI state
  darkMode: boolean;
  toggleDarkMode: () => void;
  
  sidebarCollapsed: boolean;
  toggleSidebar: () => void;
}

export const useAppStore = create<AppStore>((set) => ({
  // User
  userProfile: null,
  setUserProfile: (profile) => set({ userProfile: profile }),

  // Peers
  peers: [],
  addPeer: (peer) =>
    set((state) => ({
      peers: [...state.peers.filter((p) => p.profile.user_id !== peer.profile.user_id), peer],
    })),
  removePeer: (userId) =>
    set((state) => ({
      peers: state.peers.filter((p) => p.profile.user_id !== userId),
    })),
  updatePeer: (peer) =>
    set((state) => ({
      peers: state.peers.map((p) =>
        p.profile.user_id === peer.profile.user_id ? peer : p
      ),
    })),

  // Messages
  messages: [],
  addMessage: (message) =>
    set((state) => ({
      messages: [...state.messages, message],
    })),
  setMessages: (messages) => set({ messages }),

  // Active chat
  activeChatId: null,
  setActiveChatId: (userId) => set({ activeChatId: userId }),

  // File transfers
  fileTransfers: [],
  addFileTransfer: (transfer) =>
    set((state) => ({
      fileTransfers: [...state.fileTransfers, transfer],
    })),
  updateFileTransfer: (transfer) =>
    set((state) => ({
      fileTransfers: state.fileTransfers.map((t) =>
        t.transfer_id === transfer.transfer_id ? transfer : t
      ),
    })),

  // UI state
  darkMode: true,
  toggleDarkMode: () => set((state) => ({ darkMode: !state.darkMode })),
  
  sidebarCollapsed: false,
  toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
}));
