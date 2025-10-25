export interface UserProfile {
  user_id: string;
  username: string;
  display_name: string;
  status: UserStatus;
  status_message?: string;
  avatar_hash?: string;
}

export enum UserStatus {
  Online = "Online",
  Away = "Away",
  Busy = "Busy",
  Offline = "Offline",
}

export interface Peer {
  profile: UserProfile;
  address: {
    ip: string;
    port: number;
  };
  last_seen: string;
  public_key?: number[];
}

export interface Message {
  id: string;
  session_id: string;
  sender_id: string;
  recipient_id: string;
  message_type: MessageType;
  content: string;
  timestamp: string;
  status: MessageStatus;
  encrypted: boolean;
}

export enum MessageType {
  Text = "Text",
  Image = "Image",
  File = "File",
  Audio = "Audio",
  Video = "Video",
  System = "System",
}

export enum MessageStatus {
  Sending = "Sending",
  Sent = "Sent",
  Delivered = "Delivered",
  Read = "Read",
  Failed = "Failed",
}

export interface FileTransfer {
  transfer_id: string;
  sender_id: string;
  recipient_id: string;
  file_name: string;
  file_size: number;
  file_hash: string;
  bytes_transferred: number;
  status: TransferStatus;
  error?: string;
}

export enum TransferStatus {
  Pending = "Pending",
  Accepted = "Accepted",
  InProgress = "InProgress",
  Paused = "Paused",
  Completed = "Completed",
  Failed = "Failed",
  Cancelled = "Cancelled",
}

export interface ChatEvent {
  type: string;
  data: any;
}
