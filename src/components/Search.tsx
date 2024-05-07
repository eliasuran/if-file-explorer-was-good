import { invoke } from '@tauri-apps/api';
import { Dispatch, SetStateAction, useState } from 'react';
import { FileData } from '../lib/types';

export default function Search(props: {
  path: string[];
  setFiles: Dispatch<SetStateAction<FileData[]>>;
}) {
  const [q, setQ] = useState('');
  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        invoke('search', { q, path: props.path.join('/') }).then((res) => {
          const data = res as FileData[];

          props.setFiles(data);
        });
      }}
    >
      <input type='text' onChange={(e) => setQ(e.target.value)} className='' />
    </form>
  );
}
