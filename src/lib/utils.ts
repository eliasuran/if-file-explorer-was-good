import { invoke } from '@tauri-apps/api';
import { Dispatch, SetStateAction } from 'react';
import { FileData, OpenDirReturn } from './types';

export function openFile(path: string) {
  invoke('open_file', { path })
    .then((res) => console.log(res))
    .catch((err) => console.log(err));
}

export function openDir(
  path: string,
  setFiles: Dispatch<SetStateAction<FileData[]>>,
  setFullPath: Dispatch<SetStateAction<string[]>>,
) {
  invoke('open_dir', { fullPath: path })
    .then((res) => {
      const data = res as OpenDirReturn;
      setFiles(data.file_data);
      setFullPath(data.current_path.split('/'));
    })
    .catch((err) => console.log(err));
}

export function openRoot(
  setFiles: Dispatch<SetStateAction<FileData[]>>,
  setFullPath: Dispatch<SetStateAction<string[]>>,
) {
  invoke('open_root')
    .then((res) => {
      const data = res as OpenDirReturn;
      setFiles(data.file_data);
      setFullPath(data.current_path.split('/'));
    })
    .catch((err) => console.log(err));
}

export function findPath(selected: string, fullPath: string[]): string {
  for (let i in fullPath) {
    if (fullPath[i] === selected) {
      return fullPath.join('/').split(selected)[0] + selected + '/';
    }
  }
  return '';
}
