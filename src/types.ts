export type Config = {
  id: string;
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
  id: string;
  name: string;
  instructions: string;
  config_id: number;
  created_at: string;
  updated_at: string;
};
