import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import {
  UserProfile,
  Peer,
  Message,
  FileTransfer,
} from "./types";

export async function initializeApp(
  username: string,
  displayName: string
): Promise<UserProfile> {
  return await invoke("initialize_app", {
    request: {
      username,
      display_name: displayName,
    },
  });
}

export async function getUserProfile(): Promise<UserProfile | null> {
  return await invoke("get_user_profile");
}

export async function updateUserProfile(
  profile: UserProfile
): Promise<void> {
  await invoke("update_user_profile", { profile });
}

export async function getPeers(): Promise<Peer[]> {
  return await invoke("get_peers");
}

export async function sendMessage(
  recipientId: string,
  content: string
): Promise<Message> {
  return await invoke("send_message", {
    request: {
      recipient_id: recipientId,
      content,
    },
  });
}

export async function getMessages(): Promise<Message[]> {
  return await invoke("get_messages");
}

export async function sendFile(
  recipientId: string,
  filePath: string
): Promise<string> {
  return await invoke("send_file", {
    request: {
      recipient_id: recipientId,
      file_path: filePath,
    },
  });
}

export async function acceptFileTransfer(transferId: string): Promise<void> {
  await invoke("accept_file_transfer", { transferId });
}

export async function rejectFileTransfer(transferId: string): Promise<void> {
  await invoke("reject_file_transfer", { transferId });
}

export async function getFileTransfers(): Promise<FileTransfer[]> {
  return await invoke("get_file_transfers");
}

// Event listeners
export function listenToPeerDiscovered(
  callback: (peer: Peer) => void
): Promise<() => void> {
  return listen("peer-discovered", (event) => {
    callback(event.payload as Peer);
  });
}

export function listenToMessageReceived(
  callback: (message: Message) => void
): Promise<() => void> {
  return listen("message-received", (event) => {
    callback(event.payload as Message);
  });
}

export function listenToFileTransferRequested(
  callback: (data: any) => void
): Promise<() => void> {
  return listen("file-transfer-requested", (event) => {
    callback(event.payload);
  });
}
