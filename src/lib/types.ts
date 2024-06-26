export interface OpenDirReturn {
  current_path: string;
  file_data: FileData[];
}

export interface FileData {
  name: string;
  full_path: string;
  file_type: string;
  is_dot_file: boolean;
}
