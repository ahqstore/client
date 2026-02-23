export interface Body {
    model: "openai/gpt-4.1-mini";
    messages: Message[];
    max_tokens: number;
    response_format: {
        type: "json_schema";
        json_schema: Object;
    };
}
export interface Message {
    role: "assistant" | "developer" | "system" | "user";
    content: string;
}
export interface Response {
    choices: Array<{
        message: {
            /** The content of the message. */
            content: string;
            /** The role of the message. */
            role: string;
        };
    }>;
}
//# sourceMappingURL=type.d.ts.map