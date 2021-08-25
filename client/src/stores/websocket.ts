import { writable } from "svelte/store";

const messageStore = writable<string>("");

let socket: WebSocket | null = null;

const create = (endpoint: String) => {
  socket = new WebSocket(`ws://localhost:3000/ws${endpoint}`);

  socket.onopen = (event) => {
    socket.onmessage = (event) => {
      messageStore.set(event.data);
    }
  }
}

const destroy = () => {
  socket.close()
  socket = null
  messageStore.set("");
}

const sendMessage = (message: string) => {
  if (socket != null) {
    if (socket.readyState <= 1) {
      socket.send(message);
    }
  }
};

export default {
  subscribe: messageStore.subscribe,
  create,
  sendMessage,
  destroy,
};
