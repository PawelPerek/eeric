import * as monaco from "./monaco";

declare global {
  interface Window { monacoBridge: any; }
}

window.monacoBridge = monaco;

