import { FileData } from '../lib/types';

export default function Tooltip(props: { data: FileData }) {
  return (
    <div className='tooltip'>
      {props.data.full_path}, type: {props.data.file_type}
    </div>
  );
}
