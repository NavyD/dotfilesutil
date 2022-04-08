use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    time::Duration, thread,
};

use anyhow::{bail, Result};
use crossbeam::channel::unbounded;
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;

// trait Sender<T> {
//     fn try_send(&self, msg: T) -> Result<(), T>;
// }

// trait Receiver<T> {
//     fn recv(&self) -> Result<T>;

//     fn try_recv(&self) -> Result<T>;
// }

// impl<T> Sender<T> for std::sync::mpsc::Sender<T> {
//     fn try_send(&self, msg: T) -> Result<(), T> {
//         self.send(msg).map_err(|e| e.0)
//     }
// }

fn trigger_on_change<F>(f: F) -> Result<()>
where
    F: Fn(),
{
    todo!()
}

struct Config {
    delay: Duration,
    directories: Vec<PathBuf>,
}

fn t<M: Notifiable>(m: M) -> Result<()> {
    m.notify("test")?;
    todo!()
}

trait Monitor {
    type Iter: Iterator<Item = (Option<PathBuf>, Option<PathBuf>)>;

    fn watch(&self) -> Result<Self::Iter>;
}

struct IMonitor {
    rx: Receiver<DebouncedEvent>,
    watcher: Mutex<RecommendedWatcher>,
    config: Config,
}

impl IMonitor {
    pub fn new(config: Config) -> Self {
        // let (tx, rx) = unbounded();
        let (tx, rx) = channel();
        Self {
            watcher: Mutex::new(
                watcher(tx, config.delay.max(Duration::from_secs(2))).expect("watcher failed"),
            ),
            config,
            rx,
        }
    }
}

impl Monitor for IMonitor {
    type Iter = Box<dyn Iterator<Item = (Option<PathBuf>, Option<PathBuf>)>>;

    fn watch(&self) -> Result<Self::Iter> {
        {
            let mut watcher = self.watcher.lock().unwrap();
            for p in &self.config.directories {
                watcher.watch(p, RecursiveMode::Recursive)?;
            }
        }
    
        // let a = it.into_iter().flat_map(|e| match e {
        //     DebouncedEvent::Write(p) => Some((Some(p.clone()), Some(p))),
        //     DebouncedEvent::Remove(p) => Some((None, Some(p))),
        //     DebouncedEvent::Rename(src, dst) => Some((Some(dst), Some(src))),
        //     _ => None,
        // });
        // let a = self.rx.iter().flat_map(|e| match e {
        //     DebouncedEvent::Write(p) => Some((Some(p.clone()), Some(p))),
        //     DebouncedEvent::Remove(p) => Some((None, Some(p))),
        //     DebouncedEvent::Rename(src, dst) => Some((Some(dst), Some(src))),
        //     _ => None,
        // });
        // Ok(Box::new(a))
        todo!()
    }
}
trait Notifiable {
    fn notify<T: ToString>(&self, msg: T) -> Result<()>;
}

struct WslNotifiable;

impl Notifiable for WslNotifiable {
    fn notify<T>(&self, msg: T) -> Result<()>
    where
        T: ToString,
    {
        let name = "powershell.exe";
        let arg = format!(
            r#"
$ToastHeader = New-BTHeader -Id '001' -Title 'dotfiles notification'
New-BurntToastNotification -Text '{}' -Header $ToastHeader"#,
            msg.to_string()
        );

        Command::new(name)
            .arg(&arg)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()
            .map_err(Into::into)
            .and_then(|status| {
                if !status.success() {
                    bail!("failed to run with {} status: {} {}", status, name, arg)
                } else {
                    Ok(())
                }
            })
    }
}

struct Program<N, M> {
    monitor: M,
    notifiable: N,
}

impl<N, M> Program<N, M>
where
    N: Notifiable,
    M: Monitor,
{
    fn handle(&self) -> Result<()> {
        // 1. monitor
        for (new, old) in self.monitor.watch()? {
            match (new, old) {
                // new file
                (Some(new), None) => {
                    self.notifiable
                        .notify(format!("new file: {}", new.display()))?;
                }
                (Some(new), Some(old)) => {
                    // update
                    if new == old {
                        self.notifiable
                            .notify(format!("updated {}", new.display()))?;
                    }
                    // rename and update
                    else {
                        self.notifiable.notify(format!(
                            "rename {} to {}",
                            old.display(),
                            new.display()
                        ))?;
                    }
                }
                // delete file
                (None, Some(old)) => self
                    .notifiable
                    .notify(format!("delete {}", old.display()))?,
                (None, None) => bail!(""),
            }
        }

        todo!()
    }
}

fn git_handle<P: AsRef<Path>>(new: Option<P>, old: Option<P>) -> Result<()> {
    // match (new, old) {
    //     // new file
    //     (Some(new), None) => {}
    //     (Some(new), Some(old)) => {
    //         // update
    //         if new == old {
    //         }
    //         // rename and update
    //         else {
    //         }
    //     }
    //     // delete file
    //     (None, Some(old)) => {}
    //     (None, None) => bail!(""),
    // }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
