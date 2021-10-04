use std::fs;
use std::io;
use std::time::Duration;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, sleep, JoinHandle},
};

pub fn recurse_dir(dir: PathBuf) -> impl Iterator<Item = io::Result<PathBuf>> {
    let mut unvisited = VecDeque::new();
    unvisited.push_back(dir.clone());
    //    let mut backup_iter: Option<fs::ReadDir> = None;
    let mut backup_iter = None.into_iter().flatten();
    let mut dir = dir;
    std::iter::from_fn(move || loop {
        match backup_iter.next() {
            None => {
                dir = unvisited.pop_front()?;
                match fs::read_dir(&dir) {
                    Ok(read_dir) => backup_iter = Some(read_dir).into_iter().flatten(),
                    Err(e) => return Some(Err(e)),
                };
            }
            Some(entry) => match entry {
                Ok(dir_entry) => match dir_entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            unvisited.push_back(dir_entry.path());
                            return Some(Ok(dir_entry.path()));
                        } else if file_type.is_file() {
                            return Some(Ok(dir_entry.path()));
                        }
                    }
                    Err(e) => return Some(Err(e)),
                },
                Err(e) => return Some(Err(e)),
            },
        }
    })
}
/////////////////////////////////////////////////////////////////////////////////////
pub fn par_recurse(dir: PathBuf, threads: usize) -> Vec<io::Result<PathBuf>> {
    enum Signal {
        Waiting,
        Working,
        Answering(io::Result<PathBuf>),
    }
    let mut rxs = VecDeque::<Receiver<Option<PathBuf>>>::new();
    let mut txs = Vec::<Sender<Option<PathBuf>>>::new();
    let mut handles = Vec::<JoinHandle<()>>::new();
    let (main_tx, main_rx) = channel::<Signal>();
    let mut result = Vec::<io::Result<PathBuf>>::new();
    // each has threads tx and the rx
    for _ in 0..threads {
        let (tx, rx) = channel::<Option<PathBuf>>();
        rxs.push_back(rx);
        txs.push(tx);
    }
    txs[0].send(Some(dir)).unwrap();
    for id in 0..threads {
        let rx = rxs.pop_front().unwrap();
        let txs = txs.clone();
        let main_tx = main_tx.clone();
        handles.push(thread::spawn(move || {
            let mut step = (0..threads).map(|x: usize| (x + id) % threads).cycle();
            loop {
                main_tx.send(Signal::Waiting).unwrap();
                let rv = rx.recv();
                // очень узкое место без синхронизации
                main_tx.send(Signal::Working).unwrap();
                match rv.unwrap() {
                    None => {
                        break;
                    }
                    Some(dir) => match std::fs::read_dir(&dir) {
                        Ok(rd) => {
                            for dir in rd {
                                let entry = dir.unwrap();
                                if (&entry).file_type().unwrap().is_dir() {
                                    txs[step.next().unwrap()].send(Some(entry.path()));
                                    main_tx.send(Signal::Answering(Ok(entry.path()))).unwrap();
                                } else if (&entry).file_type().unwrap().is_file() {
                                    main_tx.send(Signal::Answering(Ok(entry.path()))).unwrap();
                                }
                            }
                        }
                        Err(e) => {
                            main_tx.send(Signal::Answering(Err(e))).unwrap();
                        }
                    },
                }
            }
        }));
    }

    let mut waiters_count = 0;
    loop {
        loop {
            match main_rx.recv().unwrap() {
                Signal::Waiting => waiters_count += 1,
                Signal::Working => waiters_count -= 1,
                Signal::Answering(entry) => result.push(entry),
            }
            if waiters_count >= threads {
                break;
            }
        }
        sleep(Duration::from_micros(100)); //kill me pls

        match main_rx.try_recv() {
            Err(_) => break,
            Ok(Signal::Waiting) => waiters_count += 1,
            Ok(Signal::Working) => waiters_count -= 1,
            Ok(Signal::Answering(entry)) => result.push(entry),
        }

    }

    for id in 0..threads {
        txs[id].send(None).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    result
}