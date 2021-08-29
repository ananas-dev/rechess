import { dev } from "$app/env";
import { devServerHost, devServerPort } from "./env";

const wsBuilder = (enpoint: string): WebSocket =>
  new WebSocket(
    (window.location.protocol === "https:" ? "wss://" : "ws://") +
      (dev ? `${devServerHost}:${devServerPort}` : window.location.host) +
      "/ws" +
      enpoint
  );

export { wsBuilder };
