use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::marker::Copy;

pub struct WorkerState{

    _worker_handle : Option<JoinHandle<()>>,
    _sender :  Option<Sender<i32>>
}

impl WorkerState{

    pub fn new() -> WorkerState{
        WorkerState{
            _worker_handle : Option::None,
            _sender : Option::None,
        }
    }

    pub fn StartWork(&mut self) -> Result<(), String> {
        match self._worker_handle {
            None => {
                println!("At start of work function");
                let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
                self._sender = Option::Some(tx);
                self._worker_handle = Option::Some(thread::spawn( move || {
                    loop{
                        println!("Waiting for input");
                        match rx.try_recv(){
                            Ok(val) => 
                            {
                                if val == 1 {
                                    return;
                                }        
                                println!("Do something");
                            },
                            _ => (),
                        }
                        thread::sleep(Duration::from_millis(1000));
                    }
                }));
                Ok(())
            },
            _ => Err("Thread already running".to_string()),
        }
    }

    pub fn EndWork(&mut self) -> Result<(), String>{
        match self._sender.as_mut(){
            Some(value) => {
                println!("Attempting to end work thread");
                value.send(1).unwrap();
                self._worker_handle.take().map(JoinHandle::join);
                Ok(())
            },
            None => Err("Failed to join thread".to_string())
        }
    }
}