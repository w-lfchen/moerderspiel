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
