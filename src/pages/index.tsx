import { useState } from "react";

// Tauri
import { invoke } from "@tauri-apps/api/tauri";

export default function Home() {
  const [isLoading, setIsLoading] = useState(false);
  const handleSubmit = async () => {
    await invoke("generate_query", { user_text: "" });
  };

  return (
    <section className="flex items-center w-full min-h-screen justify-center bg-gray-800 flex-col font-poppins">
      <article className="w-full flex items-center justify-center">
        <form
          className="flex flex-col max-w-xl w-full gap-4"
          onSubmit={handleSubmit}
        >
          <textarea
            placeholder="Search SQL Query.."
            className="rounded-md p-3 w-full border-none outline-none"
            required
          />
          <button className="bg-slate-900 text-white w-full p-3 rounded-md outline-none hover:bg-slate-950 transition-all ease-in-out duration-500">
            Submit
          </button>
        </form>
      </article>

      <article>
        <div>Loading...</div>
      </article>
    </section>
  );
}
``;
