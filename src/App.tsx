import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';

function App() {
  const [showDotFiles, setShowDotFiles] = useState(false);
  const [files, setFiles] = useState([] as string[][]);

  useEffect(() => {
    invoke('read_fs').then((files) => setFiles(files as string[][]));
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
              <span>{file[0]}</span>
            ) : (
              file[0].split('/')[3][0] !== '.' && <span>{file[0]}</span>
            )}
          </>
        ))}
      </div>
    </main>
  );
}

export default App;
