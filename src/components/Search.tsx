import { invoke } from '@tauri-apps/api';
import { Dispatch, SetStateAction, useState } from 'react';
import { FileData } from '../lib/types';
import { listen } from '@tauri-apps/api/event';

export default function Search(props: {
  path: string[];
  setFiles: Dispatch<SetStateAction<FileData[]>>;
}) {
  const [q, setQ] = useState('');
  return (
    <form
      onSubmit={async (e) => {
        e.preventDefault();
        invoke('search_files', { q, path: props.path.join('/') });
        startSerialEventListener(props.setFiles);
      }}
    >
      <input type='text' onChange={(e) => setQ(e.target.value)} className='' />
    </form>
  );
}

interface Payload {
  data: FileData[];
  done: boolean;
}

async function startSerialEventListener(
  setFiles: Dispatch<SetStateAction<FileData[]>>,
) {
  const unlisten = await listen<Payload>('incoming-data', (event) => {
    if (event.payload.done) {
      console.log('Done searching');
      unlisten;
      return;
    }
    setFiles(event.payload.data);
  });
  return unlisten;
}
