import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// TODO: adjust
export const backend_url = "https://localhost:3000";

// post json body to backend path
export function fetch_post(path: string, body?: object): Promise<Response> {
  if (body === undefined) {
    return fetch(backend_url + path, { method: "POST" });
  }
  return fetch(backend_url + path, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
}

// get
export function fetch_get(path: string): Promise<Response> {
  return fetch(backend_url + path, { method: "GET" });
}

// hashed s with sha-256 and returns a string
// see https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest
export async function digestMessage(message: string): Promise<string> {
  const msgUint8 = new TextEncoder().encode(message);
  const hashBuffer = await window.crypto.subtle.digest("SHA-256", msgUint8);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  // toHex is giving me errors :(
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  return hashHex;
}
