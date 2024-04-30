import { invoke } from '@tauri-apps/api/tauri';
import { useEffect } from 'react';

function App() {
  useEffect(() => {
    invoke('read_fs').then((fs) => console.log(fs));
  }, []);
  return <div></div>;
}

export default App;
