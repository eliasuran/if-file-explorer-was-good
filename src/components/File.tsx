import { Dispatch, SetStateAction, useState } from 'react';
import { FileData } from '../lib/types';
import { openDir, openFile } from '../lib/utils';
import { Icon } from '@iconify/react/dist/iconify.js';
import Tooltip from './Tooltip';

export default function File(props: {
  data: FileData;
  setFiles: Dispatch<SetStateAction<FileData[]>>;
  setCurrentPath: Dispatch<SetStateAction<string[]>>;
}) {
  const [showTooltip, setShowTooltip] = useState(false);
  return (
    <button
      key={props.data.full_path}
      onMouseEnter={() => setShowTooltip(true)}
      onMouseLeave={() => setShowTooltip(false)}
      onClick={() =>
        props.data.file_type === 'dir'
          ? openDir(props.data.full_path, props.setFiles, props.setCurrentPath)
          : props.data.file_type === 'file'
            ? openFile(props.data.full_path)
            : console.log('Unknown file, cannot open')
      }
    >
      {showTooltip && <Tooltip data={props.data} />}
      <Icon
        height={15}
        icon={
          props.data.file_type === 'file'
            ? 'ic:outline-insert-drive-file'
            : props.data.file_type === 'dir'
              ? 'ic:outline-folder'
              : 'ic:baseline-question-mark'
        }
      />
      {props.data.name}
    </button>
  );
}
