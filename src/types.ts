export type Config = {
  id: number;
  temperature: number;
  num_ctx: number;
  frequency_penalty: number;
  presence_penalty: number;
};

export type FileHistory = {
  id: number;
  file_id: string;
  name: string;
  created_at: string;
  updated_at: string;
};

export type Assistant = {
  id: number;
  name: string;
  instructions: string;
  config_id: number;
  created_at: string;
  updated_at: string;
};

export type ChatHistory = {
  role: string;
  content: string;
}[];

export type State = {
  model: string;
  prompt: string;
  response: string;
  assistant: Assistant | null;
  config: Config | null;
};
