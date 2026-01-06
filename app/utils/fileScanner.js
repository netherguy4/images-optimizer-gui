import { stat, readDir } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import allowedExtensions from '../configs/supportedExt';

const isSupported = (fileName) => {
  if (!fileName || !fileName.includes('.')) return false;
  const ext = fileName.split('.').pop().toLowerCase();
  return allowedExtensions.includes(ext);
};

const buildFileTree = async (currentPath) => {
  const name = currentPath.split(/[/\\]/).pop();
  let metadata;

  try {
    metadata = await stat(currentPath);
  } catch (e) {
    console.error(`Failed to stat ${currentPath}`, e);
    return null;
  }

  if (!metadata.isDirectory) {
    if (isSupported(name)) {
      return {
        id: crypto.randomUUID(),
        name: name,
        path: currentPath,
        type: 'file',
        size: metadata.size,
        extension: name.split('.').pop(),
        fileCount: 1,
      };
    }
    return null;
  }

  let children = [];
  let totalSize = 0;
  let fileCount = 0;

  try {
    const entries = await readDir(currentPath);

    const promises = entries.map(async (entry) => {
      if (entry.isFile && !isSupported(entry.name)) {
        return null;
      }

      const childPath = await join(currentPath, entry.name);
      return buildFileTree(childPath);
    });

    const results = await Promise.all(promises);

    for (const childNode of results) {
      if (childNode) {
        children.push(childNode);
        totalSize += childNode.size;
        fileCount += childNode.type === 'folder' ? childNode.fileCount || 0 : 1;
      }
    }
  } catch (err) {
    console.error(`Error reading dir ${currentPath}`, err);
    return null;
  }

  if (children.length === 0) return null;

  return {
    id: crypto.randomUUID(),
    name: name,
    path: currentPath,
    type: 'folder',
    size: totalSize,
    fileCount: fileCount,
    children: children,
    isOpen: false,
  };
};

export const processPaths = async (paths) => {
  const promises = paths.map((path) => buildFileTree(path));
  const results = await Promise.all(promises);

  return results.filter((node) => node !== null);
};
