import { useEffect, useState } from 'react';
import { FileData } from './lib/types';
import { findPath, openDir, openRoot } from './lib/utils';
import Search from './components/Search';
import File from './components/File';

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
          <File
            key={file.full_path}
            data={file}
            setFiles={setFiles}
            setCurrentPath={setCurrentPath}
          />
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
