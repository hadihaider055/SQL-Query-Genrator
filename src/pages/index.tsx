import { useState } from "react";

// Next
import Link from "next/link";

// Tauri
import { invoke } from "@tauri-apps/api/tauri";

// React Syntax Highlighter
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { darcula } from "react-syntax-highlighter/dist/esm/styles/prism";

// Sweet Alert
import Swal from "sweetalert2";

type OAIResponseType = {
  result: ResponseResultType;
  prompt: string;
  max_tokens: number;
};

type ResponseChoicesType = {
  text: string;
  index: number;
  finish_reason: string;
};

type ResponseResultType = {
  Ok: {
    choices: ResponseChoicesType[];
    id: string;
    object: string;
    created: number;
    model: string;
  };
};

export default function Home() {
  const [isLoading, setIsLoading] = useState(false);
  const [userText, setUserText] = useState("");
  const [query, setQuery] = useState<any>(null);

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setQuery(null);
    setIsLoading(true);

    try {
      const res: any = await invoke("generate_query", { userText });
      setQuery(JSON.parse(res?.result?.Ok));
    } catch (err: any) {
      console.log(err);
      Swal.fire({
        icon: "error",
        title: "Oops...",
        text: err || "Something went wrong!",
      });
    }

    setIsLoading(false);
  };

  return (
    <section className="w-full h-full font-poppins flex justify-center items-center flex-col">
      <div className="absolute top-1/2 -mt-32 pb-14 wax-w-xl w-full">
        <h1 className="text-gray-300 font-bold text-center text-3xl -mt-4 mb-10 tracking-wider">
          SQL Query Generator
        </h1>
        <article className="w-full flex items-center justify-center">
          <form
            className="flex flex-col max-w-xl w-full gap-4"
            onSubmit={handleSubmit}
          >
            <textarea
              placeholder="Search SQL Query.."
              className="rounded-md p-3 w-full border-none outline-none disabled:cursor-wait disabled:opacity-70"
              required
              onChange={(e) => setUserText(e.target.value)}
              value={userText}
              disabled={isLoading}
            />
            <button
              className="bg-slate-900 text-white w-full p-3 rounded-md outline-none hover:bg-slate-950 transition-all ease-in-out duration-500 disabled:cursor-not-allowed disabled:opacity-80"
              disabled={isLoading}
            >
              {isLoading ? "Searching" : "Search"}
            </button>
          </form>
        </article>

        <article className="my-10 flex flex-col items-center w-full">
          {isLoading && (
            <div className="w-40">
              <img src="/images/cat.gif" />
            </div>
          )}
          {query && (
            <div className="bg-gray-400 p-3 rounded-md w-full max-w-xl">
              {query?.choices?.map((item: ResponseChoicesType) => (
                <SyntaxHighlighter
                  language="sql"
                  key={item?.index}
                  style={darcula}
                >
                  {item?.text}
                </SyntaxHighlighter>
              ))}
            </div>
          )}
          <p></p>
        </article>
      </div>
      <div className="fixed bottom-0 py-3 left-1/2 -translate-x-1/2 bg-slate-900 w-full flex items-center justify-center">
        <h4 className="text-gray-300 text-lg tracking-wide">
          Made with ❤️ by{" "}
          <Link href="https://github.com/hadihaider055" className="underline">
            Hadi Haider
          </Link>
        </h4>
      </div>
    </section>
  );
}
``;
