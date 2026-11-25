use super::*;

fn library(dir: &tempfile::TempDir) -> Library {
  Library::new(dir.path().join("data"), dir.path().join("Müsic")).unwrap()
}

/// Contents differ between recordings; sizes may not.
fn add(library: &Library, dir: &tempfile::TempDir, title: &str, bytes: usize) -> Recording {
  let source = dir.path().join("take.wav");
  fs::write(&source, vec![title.as_bytes()[0]; bytes]).unwrap();
  let new = NewRecording { kind: Kind::Voice, title, source: &source, duration_ms: 1000, peaks: vec![0.5] };
  let recording = library.add(new).unwrap();
  assert!(!source.exists());
  recording
}

#[test]
fn survives_restart() {
  let dir = tempfile::tempdir().unwrap();
  let first = add(&library(&dir), &dir, "First", 10);
  let second = add(&library(&dir), &dir, "Second", 11);
  let list = library(&dir).list().unwrap();
  assert_eq!(list.iter().map(|item| item.id).collect::<Vec<_>>(), [second.id, first.id]);
  assert!(list.iter().all(|item| item.available));
  assert_eq!(list[1].title, "First");
}

#[test]
fn renames_and_deletes() {
  let dir = tempfile::tempdir().unwrap();
  let library = library(&dir);
  let id = add(&library, &dir, "Old", 10).id;
  library.rename(id, "  New\nname ").unwrap();
  let (path, title) = library.open(id).unwrap();
  assert_eq!(title, "New name");
  assert!(library.rename(id, "  ").is_err());
  library.delete(id).unwrap();
  assert!(!path.exists() && library.list().unwrap().is_empty());
  assert_eq!(library.open(id), Err(LibraryError::Missing));
  // Ids are never reused.
  assert!(add(&library, &dir, "Next", 10).id > id);
}

#[test]
fn relinks_moved_files_together() {
  let dir = tempfile::tempdir().unwrap();
  let library = library(&dir);
  let ids = [("A", 10), ("B", 11)].map(|(title, bytes)| add(&library, &dir, title, bytes).id);
  let moved = dir.path().join("moved");
  fs::create_dir(&moved).unwrap();
  for id in ids {
    let (path, _) = library.open(id).unwrap();
    fs::rename(&path, moved.join(path.file_name().unwrap())).unwrap();
  }
  assert!(library.list().unwrap().iter().all(|item| !item.available));
  assert_eq!(library.open(ids[0]), Err(LibraryError::Missing));

  let wrong = dir.path().join("other.wav");
  fs::write(&wrong, "x").unwrap();
  assert_eq!(library.relink(ids[0], &wrong), Err(LibraryError::Mismatch));
  library.relink(ids[0], &moved.join("A.wav")).unwrap();
  assert!(library.list().unwrap().iter().all(|item| item.available));
}

#[test]
fn finds_files_put_back_and_deletes_missing_entries() {
  let dir = tempfile::tempdir().unwrap();
  let library = library(&dir);
  let id = add(&library, &dir, "A", 10).id;
  let (path, _) = library.open(id).unwrap();
  // A different file under the same name is not this recording, and is never deleted.
  fs::write(&path, "someone else's").unwrap();
  assert!(!library.list().unwrap()[0].available);
  library.delete(id).unwrap();
  assert!(path.exists());
}

#[test]
fn tells_same_sized_tracks_apart() {
  let dir = tempfile::tempdir().unwrap();
  let library = library(&dir);
  let old = add(&library, &dir, "Alpha", 10).id;
  let (path, _) = library.open(old).unwrap();
  fs::rename(&path, dir.path().join("elsewhere.wav")).unwrap();
  // A new recording takes the freed file name.
  let source = dir.path().join("take.wav");
  fs::write(&source, vec![9; 10]).unwrap();
  let new = NewRecording { kind: Kind::Music, title: "Alpha", source: &source, duration_ms: 1, peaks: vec![] };
  let new = library.add(new).unwrap().id;
  assert_eq!(library.open(new).unwrap().0, path);
  assert_eq!(library.open(old), Err(LibraryError::Missing));
  assert_eq!(library.relink(old, &path), Err(LibraryError::Mismatch));
  library.delete(old).unwrap();
  assert!(path.exists());
}

#[test]
fn recovers_from_a_damaged_index() {
  let dir = tempfile::tempdir().unwrap();
  let library = library(&dir);
  add(&library, &dir, "A", 10);
  add(&library, &dir, "B", 11);
  fs::write(dir.path().join("data/library.json"), "{broken").unwrap();
  let titles: Vec<_> = library.list().unwrap().into_iter().map(|item| item.title).collect();
  assert_eq!(titles, ["A"]);
  assert!(dir.path().join("data/library.damaged").exists());
  add(&library, &dir, "C", 12);
  assert_eq!(library.list().unwrap().len(), 2);
}

#[test]
fn uses_the_fallback_folder() {
  let dir = tempfile::tempdir().unwrap();
  // A file where the folder should be.
  fs::write(dir.path().join("Müsic"), "x").unwrap();
  let library = library(&dir);
  let id = add(&library, &dir, "A", 10).id;
  assert!(library.open(id).unwrap().0.starts_with(dir.path().join("data")));
}

#[test]
fn windows_can_add_at_the_same_time() {
  let dir = tempfile::tempdir().unwrap();
  let library = library(&dir);
  std::thread::scope(|scope| {
    for window in 0..4 {
      let (library, dir) = (library.clone(), &dir);
      scope.spawn(move || {
        for number in 0..5 {
          let source = dir.path().join(format!("{window}.wav"));
          fs::write(&source, vec![1; 10 + number]).unwrap();
          let new = NewRecording { kind: Kind::Music, title: "Same", source: &source, duration_ms: 1, peaks: vec![] };
          library.add(new).unwrap();
        }
      });
    }
  });
  let mut ids: Vec<_> = library.list().unwrap().iter().map(|item| item.id).collect();
  ids.sort_unstable();
  ids.dedup();
  assert_eq!(ids.len(), 20);
}
