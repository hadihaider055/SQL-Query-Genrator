import Image from "next/image";
import { Inter } from "next/font/google";
import { invoke } from "@tauri-apps/api/tauri";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  return <button>Click me!</button>;
}
