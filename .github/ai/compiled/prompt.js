var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import { join } from "node:path";
import { readFileSync } from "node:fs";
const data = join(import.meta.dirname, "../prompt.schema.json");
const schema = JSON.parse(readFileSync(data).toString());
export const handle = (text) => __awaiter(void 0, void 0, void 0, function* () {
    const path = "https://models.github.ai/orgs/ahqstore/inference/chat/completions";
    const response = yield fetch(path, {
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
        }, null, 2),
        headers: {
            "content-type": "application/json",
            "accept": "application/vnd.github+json",
            "User-Agent": "AHQ Store Scripts",
            "Authorization": `Bearer ${process.env["GITHUB_TOKEN"]}`
        }
    }).then((d) => d.json());
    console.log(response);
    console.log(JSON.stringify(response, null, 4));
    return JSON.parse(response.choices[0].message.content);
});
