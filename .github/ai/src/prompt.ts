import { join } from "node:path"
import { readFileSync } from "node:fs"

import type { Body, Response } from "./type.js"

const data = join(import.meta.dirname, "../prompt.schema.json");
const schema = JSON.parse(readFileSync(data).toString())

export const handle = async (text: string) => {
  const path = "https://models.github.ai/orgs/ahqstore/inference/chat/completions"

  const response = await fetch(path, {
    method: "POST",
    body: JSON.stringify({
      max_tokens: 4096,
      messages: [
        {
          content: "Classify the text as per the schema given",
          role: "system"
        },
        {
          content: text,
          role: "user"
        }
      ],
      model: "openai/gpt-4.1-mini",
      response_format: {
        type: "json_schema",
        json_schema: schema
      }
    } as Body, null, 2),
    headers: {
      "content-type": "application/json",
      "accept": "application/vnd.github+json",
      "User-Agent": "AHQ Store Scripts",
      "Authorization": `Bearer ${process.env["GITHUB_TOKEN"]}`
    }
  }).then((d) => d.json() as Promise<Response>);

  console.log(response);
  console.log(JSON.stringify(response, null, 4));

  return JSON.parse(response.choices[0].message.content);
}
