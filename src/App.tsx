import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import { FileData } from './lib/types';

function App() {
  const [showDotFiles, setShowDotFiles] = useState(false);
  const [files, setFiles] = useState([] as FileData[]);

  useEffect(() => {
    invoke('read_fs')
      .then((files) => setFiles(files as FileData[]))
      .catch((err) => console.log(err));
  }, []);

  return (
    <main>
      <button onClick={() => setShowDotFiles(!showDotFiles)}>
        {showDotFiles ? 'hide' : 'show'} dotfiles
      </button>
      <div className='file-wrapper'>
        <span
          onClick={() =>
            invoke('read_fs')
              .then((data) => setFiles(data as FileData[]))
              .catch((err) => console.log(err))
          }
        >
          ..
        </span>
        {files.map((file) => (
          <>
            {showDotFiles ? (
              <span
                onClick={() =>
                  invoke('open_dir', { fullPath: file.full_path })
                    .then((data) => setFiles(data as FileData[]))
                    .catch((err) => console.log(err))
                }
              >
                {file.name}
              </span>
            ) : (
              !file.is_dot_file && (
                <span
                  onClick={() =>
                    invoke('open_dir', { fullPath: file.full_path })
                      .then((data) => setFiles(data as FileData[]))
                      .catch((err) => console.log(err))
                  }
                >
                  {file.name}
                </span>
              )
            )}
          </>
        ))}
      </div>
    </main>
  );
}

export default App;
