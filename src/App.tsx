import { useEffect, useState } from 'react';
import { FileData } from './lib/types';
import { findPath, openDir, openFile, openRoot } from './lib/utils';
import { Icon } from '@iconify/react';
import Search from './components/Search';

function App() {
  const [currentPath, setCurrentPath] = useState([] as string[]);
  const [showDotFiles, setShowDotFiles] = useState(false);
  const [files, setFiles] = useState([] as FileData[]);

  useEffect(() => {
    openRoot(setFiles, setCurrentPath);
  }, []);

  return (
    <main>
      <Search path={currentPath} setFiles={setFiles} />
      <button onClick={() => setShowDotFiles(!showDotFiles)}>
        {showDotFiles ? 'hide' : 'show'} dotfiles
      </button>
      <div className='file-wrapper'>
        <button onClick={() => openRoot(setFiles, setCurrentPath)}>..</button>
        {files.map((file) => (
          <button
            key={file.full_path}
            onClick={() =>
              file.file_type === 'dir'
                ? openDir(file.full_path, setFiles, setCurrentPath)
                : file.file_type === 'file'
                  ? openFile(file.full_path)
                  : console.log('Unknown file, cannot open')
            }
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
        {currentPath.map(
          (section, index) =>
            section !== '' && (
              <div key={index}>
                <button
                  onClick={() => {
                    const path = findPath(section, currentPath);
                    openDir(path, setFiles, setCurrentPath);
                  }}
                  className='breadcrumb-item'
                >
                  {section}
                </button>
                {index !== currentPath.length - 1 && <span>-</span>}
              </div>
            ),
        )}
      </div>
    </main>
  );
}

export default App;
