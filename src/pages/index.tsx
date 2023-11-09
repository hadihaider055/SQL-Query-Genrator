import Image from "next/image";
import { Inter } from "next/font/google";
import { invoke } from "@tauri-apps/api/tauri";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  return (
    <button
      onClick={async () => {
        const i = await invoke("generate_query");
        console.log("i", i);
      }}
    >
      Click me!
    </button>
  );
}
