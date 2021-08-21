import { writable } from "svelte/store";

const messageStore = writable("");

const socket = new WebSocket("ws://localhost:3000/ws/test");

socket.addEventListener("open", (event) => {
  console.log("Connected to socket");
});

// Listen for messages
socket.addEventListener("message", (event) => {
  messageStore.set(event.data);
});

const send_message = (message: string) => {
  if (socket.readyState <= 1) {
    socket.send(message);
  }
};

export default {
  subscribe: messageStore.subscribe,
  send_message,
};
