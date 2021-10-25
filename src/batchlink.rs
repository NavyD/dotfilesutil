use log::{debug, error, info, trace};
use std::{
    fmt::Debug,
    ops::Deref,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use anyhow::{bail, Result};

/// 返回指定目录src下子目录到dst目录的链接。如src=/a/b, dst=/c, src下
/// 存在文件a.txt将会链接/a/b/a.txt -> /c/a.txt。
///
/// * 如果inclusive为空则默认包含所有子目录，否则仅使用inclusive目录
/// * 如果exclusive不为空则排除对应的子目录
/// * 如果recursively=false时仅链接一层目录
/// * 如果force=true则不管dst中存在对应目录
pub fn links<P: AsRef<Path> + Deref<Target = Path> + Debug>(
    src: P,
    dst: P,
    inclusive: &[P],
    exclusive: &[P],
    recursively: bool,
    force: bool,
) -> Result<Vec<(PathBuf, PathBuf)>> {
    if !src.exists() || !dst.exists() {
        bail!(
            "invalid args: a nonexistents path src={:?} or dst={:?}",
            src,
            dst
        )
    }

    if inclusive.iter().any(|p| !p.exists()) {
        let nonex = inclusive
            .iter()
            .filter(|p| !p.exists())
            .collect::<Vec<&P>>();
        error!(
            "found nonexistent directories {:?} in inclusive: {:?}",
            nonex, inclusive
        );
        bail!("invalid args: found not nonexistent dirs in inclusive or exclusive")
    }

    if exclusive.iter().any(|p| !p.exists()) {
        let nonex = inclusive
            .iter()
            .filter(|p| !p.exists())
            .collect::<Vec<&P>>();
        error!(
            "found nonexistent directories {:?} in exclusive: {:?}",
            nonex, exclusive
        );
        bail!("invalid args: found not nonexistent dirs in inclusive or exclusive")
    }

    let inclusive = inclusive
        .iter()
        .map(|p| p.canonicalize())
        .flatten()
        .collect::<Vec<PathBuf>>();
    let exclusive = exclusive
        .iter()
        .map(|p| p.canonicalize())
        .flatten()
        .collect::<Vec<PathBuf>>();

    let mut links = vec![];

    for entry in WalkDir::new(&src)
        .max_depth(if recursively { usize::MAX } else { 1 })
        .into_iter()
        .flatten()
    {
        let cur = entry.path().canonicalize()?;
        if (!exclusive.is_empty() && exclusive.iter().any(|p| cur.starts_with(p)))
            || (!inclusive.is_empty() && inclusive.iter().all(|p| !cur.starts_with(p)))
        {
            info!("skipped directory {:?}", cur);
            continue;
        }
        if !cur.is_file() {
            trace!("skipped a non file {:?}", cur);
            continue;
        }

        let rel_path = cur.strip_prefix(&src)?;
        debug!(
            "check relative path: {:?} for src: {:?}, cur: {:?}",
            rel_path, src, cur
        );
        let to = dst.join(rel_path);
        if force || !to.exists() {
            debug!("linking `{:?}` from `{:?}`", to, cur);
            links.push((cur, to))
        } else {
            info!(
                "skipped linking for a exists dst path: {:?}, src path: {:?}",
                to, cur
            );
        }
    }
    Ok(links)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{create_dir_all, File},
        io::Write,
    };
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_link() -> Result<()> {
        let root = tempdir()?;

        let src = root.path().join("src");
        let dst = root.path().join("dst");

        let rel_a = Path::new("a.txt");
        let rel_b = Path::new("a/b.txt");
        let rel_c = Path::new("a/c.txt");

        create_file(src.join(rel_a))?;
        create_file(src.join(rel_b))?;
        create_file(src.join(rel_c))?;
        create_file(dst.join(rel_a))?;

        let links = links(src.as_path(), dst.as_path(), &[], &[], true, false)?;

        assert_eq!(links.len(), 2);
        assert!(links.iter().all(|p| p.0 != src.join(rel_a)));
        assert!(links
            .iter()
            .any(|p| p.0 == src.join(rel_b) && p.1 == dst.join(rel_b)));
        assert!(links
            .iter()
            .any(|p| p.0 == src.join(rel_c) && p.1 == dst.join(rel_c)));
        Ok(())
    }

    #[test]
    fn test_link_when_in_and_ex() -> Result<()> {
        let root = tempdir()?;

        let src = root.path().join("src");
        let dst = root.path().join("dst");

        let rel_a = Path::new("a.txt");
        let rel_b = Path::new("a/b.txt");
        let rel_c = Path::new("a/c.txt");

        create_file(src.join(rel_a))?;
        create_file(src.join(rel_b))?;
        create_file(src.join(rel_c))?;
        create_file(dst.join(rel_a))?;

        let inclusive = [src.join(rel_b)];
        let exclusive = [src.join(rel_c)];

        let links = links(
            src.as_path(),
            dst.as_path(),
            &inclusive
                .iter()
                .map(PathBuf::as_path)
                .collect::<Vec<&Path>>(),
            &exclusive
                .iter()
                .map(PathBuf::as_path)
                .collect::<Vec<&Path>>(),
            true,
            false,
        )?;

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0, src.join(rel_b));
        assert_eq!(links[0].1, dst.join(rel_b));
        Ok(())
    }

    #[test]
    fn test_link_when_no_recursively() -> Result<()> {
        let root = tempdir()?;

        let src = root.path().join("src");
        let dst = root.path().join("dst");

        let rel_a = Path::new("a.txt");
        let rel_b = Path::new("a/b.txt");
        let rel_c = Path::new("a/c.txt");

        create_file(src.join(rel_a))?;
        create_file(src.join(rel_b))?;
        create_file(src.join(rel_c))?;
        create_dir_all(&dst)?;

        let links = links(src.as_path(), dst.as_path(), &[], &[], false, false)?;
        assert_eq!(links.len(), 1);
        Ok(())
    }

    fn create_file(p: PathBuf) -> Result<()> {
        create_dir_all(p.parent().unwrap())?;
        assert!(!p.exists());
        writeln!(File::create(&p)?, "path: {}", p.to_str().unwrap())?;
        assert!(p.exists());
        Ok(())
    }
}
