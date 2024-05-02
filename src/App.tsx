import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import { FileData } from './lib/types';
import { openDir } from './lib/uilts';
import { Icon } from '@iconify/react';

function App() {
  const [showDotFiles, setShowDotFiles] = useState(false);
  const [files, setFiles] = useState([] as FileData[]);

  useEffect(() => {
    invoke('read_root')
      .then((files) => setFiles(files as FileData[]))
      .catch((err) => console.log(err));
  }, []);

  return (
    <main>
      <button onClick={() => setShowDotFiles(!showDotFiles)}>
        {showDotFiles ? 'hide' : 'show'} dotfiles
      </button>
      <div className='file-wrapper'>
        <button
          onClick={() =>
            invoke('read_root')
              .then((data) => setFiles(data as FileData[]))
              .catch((err) => console.log(err))
          }
        >
          ..
        </button>
        {files.map((file) => (
          <button onClick={() => openDir(file.full_path, setFiles)}>
            <Icon
              height={15}
              icon={
                file.file_type === 'file'
                  ? 'ic:outline-insert-drive-file'
                  : file.file_type === 'dir'
                    ? 'ic:outline-folder'
                    : 'ic:baseline-question-mark'
              }
            />
            {file.name}
          </button>
        ))}
      </div>
      <div className='breadcrumb-wrapper'></div>
    </main>
  );
}

export default App;
