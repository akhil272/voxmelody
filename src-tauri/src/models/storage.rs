use std::path::{Path, PathBuf};

use super::manifest::{ModelFile, Pack};

pub fn file_path(root: &Path, pack: &Pack, file: &ModelFile) -> PathBuf {
  root.join(&pack.id).join(&file.path)
}

pub fn part_path(target: &Path) -> PathBuf {
  let mut name = target.as_os_str().to_owned();
  name.push(".part");
  PathBuf::from(name)
}

fn length(path: &Path) -> u64 {
  std::fs::metadata(path).map(|meta| meta.len()).unwrap_or(0)
}

/// Finished files are renamed into place only after their checksum passes.
pub fn is_installed(root: &Path, pack: &Pack, file: &ModelFile) -> bool {
  length(&file_path(root, pack, file)) == file.size
}

pub fn file_progress(root: &Path, pack: &Pack, file: &ModelFile) -> u64 {
  if is_installed(root, pack, file) {
    return file.size;
  }
  length(&part_path(&file_path(root, pack, file))).min(file.size)
}

pub fn pack_progress(root: &Path, pack: &Pack) -> u64 {
  pack.files.iter().map(|file| file_progress(root, pack, file)).sum()
}

/// The same file inside another installed pack.
pub fn shared_copy(root: &Path, pack: &Pack, file: &ModelFile, packs: &[Pack]) -> Option<PathBuf> {
  packs
    .iter()
    .filter(|other| other.id != pack.id)
    .flat_map(|other| other.files.iter().map(move |candidate| (other, candidate)))
    .find(|(other, candidate)| candidate.sha256 == file.sha256 && is_installed(root, other, candidate))
    .map(|(other, candidate)| file_path(root, other, candidate))
}

/// Reuses files other packs already have, so they take no extra space.
pub fn link_shared(root: &Path, pack: &Pack, packs: &[Pack]) {
  for file in pack.files.iter().filter(|file| !is_installed(root, pack, file)) {
    let Some(source) = shared_copy(root, pack, file, packs) else { continue };
    let target = file_path(root, pack, file);
    if let Some(parent) = target.parent() {
      let _ = std::fs::create_dir_all(parent);
    }
    // Drives without links download the file instead.
    if std::fs::hard_link(source, &target).is_ok() {
      let _ = std::fs::remove_file(part_path(&target));
    }
  }
}

/// Bytes a download of this pack would still fetch.
pub fn download_bytes(root: &Path, pack: &Pack, packs: &[Pack]) -> u64 {
  let missing = |file: &&ModelFile| !is_installed(root, pack, file) && shared_copy(root, pack, file, packs).is_none();
  pack.files.iter().filter(missing).map(|file| file.size - file_progress(root, pack, file)).sum()
}

pub fn is_pack_installed(root: &Path, pack: &Pack) -> bool {
  pack.files.iter().all(|file| is_installed(root, pack, file))
}

/// Free space on the volume that holds `root`, which may not exist yet.
pub fn free_space(root: &Path) -> std::io::Result<u64> {
  let existing = root.ancestors().find(|path| path.exists()).unwrap_or(root);
  fs4::available_space(existing)
}
