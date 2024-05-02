import { invoke } from '@tauri-apps/api';
import { Dispatch, SetStateAction } from 'react';
import { FileData } from './types';

export function openDir(
  path: string,
  setFiles: Dispatch<SetStateAction<FileData[]>>,
) {
  invoke('open_dir', { fullPath: path })
    .then((data) => setFiles(data as FileData[]))
    .catch((err) => console.log(err));
}
