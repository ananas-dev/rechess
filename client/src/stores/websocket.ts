import { writable } from "svelte/store";

const messageStore = writable("");

let socket: WebSocket | null = null;

const create = () => {
  socket = new WebSocket("ws://localhost:3000/ws/test");

  socket.addEventListener("open", (event) => {
    console.log("Connected to socket");
  });

  // Listen for messages
  socket.addEventListener("message", (event) => {
    messageStore.set(event.data);
  });
}

const destroy = () => {
  socket.close()
  socket = null
}

const sendMessage = (message: string) => {
  if (socket.readyState <= 1) {
    socket.send(message);
  }
};

export default {
  subscribe: messageStore.subscribe,
  create,
  sendMessage,
  destroy,
};
