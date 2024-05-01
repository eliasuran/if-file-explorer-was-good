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
        {files.map((file) => (
          <>
            {showDotFiles ? (
              <span>{file.file_path}</span>
            ) : (
              file.file_path.split('/')[3][0] !== '.' && (
                <span>{file.file_path}</span>
              )
            )}
          </>
        ))}
      </div>
    </main>
  );
}

export default App;
