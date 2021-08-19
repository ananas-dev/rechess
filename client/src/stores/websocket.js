import { writable } from "svelte/store";

const messageStore = writable("");

let socket;

const connect = (url) => {
  socket = new WebSocket(url);

  socket.addEventListener("open", (event) => {
    console.log("It's open");
  });

  // Listen for messages
  socket.addEventListener("message", (event) => {
    messageStore.set(event.data);
  });
} 

const send_message = (message) => {
  if (socket.readyState <= 1) {
    socket.send(message);
  }
};

export default {
  subscribe: messageStore.subscribe,
  connect,
  send_message,
};
