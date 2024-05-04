import { useEffect, useState } from 'react';
import { FileData } from './lib/types';
import { openDir, openRoot } from './lib/utils';
import { Icon } from '@iconify/react';

function App() {
  const [currentPath, setCurrentPath] = useState([] as string[]);
  const [showDotFiles, setShowDotFiles] = useState(false);
  const [files, setFiles] = useState([] as FileData[]);

  useEffect(() => {
    openRoot(setFiles, setCurrentPath);
  }, []);

  return (
    <main>
      <button onClick={() => setShowDotFiles(!showDotFiles)}>
        {showDotFiles ? 'hide' : 'show'} dotfiles
      </button>
      <div className='file-wrapper'>
        <button onClick={() => openRoot(setFiles, setCurrentPath)}>..</button>
        {files.map((file) => (
          <button
            key={file.full_path}
            onClick={() => openDir(file.full_path, setFiles, setCurrentPath)}
          >
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
      <div className='breadcrumb-wrapper'>
        {currentPath.filter((section) => section !== '').join(' - ')}
      </div>
    </main>
  );
}

export default App;
