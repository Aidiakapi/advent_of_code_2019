use crate::intcode::{growing_memory, util::parse_intcode, GrowingMemory, IoOperation, Value, VM};
use arrayvec::ArrayVec;
use std::collections::VecDeque;

module!(pt1: parse_intcode, pt2: parse_intcode);

#[derive(Debug, Clone)]
struct NIC {
    vm: VM<GrowingMemory>,
    output_batch: ArrayVec<[Value; 3]>,
    did_yield: bool,
}

fn init_network(input: &Vec<Value>) -> (Vec<VecDeque<Value>>, Vec<NIC>) {
    let mut input_queues = vec![VecDeque::new(); 50];
    for i in 0..50 {
        input_queues[i as usize].push_back(i as Value);
    }
    let nics = vec![
        NIC {
            vm: VM::new(growing_memory(input.clone())),
            output_batch: ArrayVec::new(),
            did_yield: false,
        };
        50
    ];
    (input_queues, nics)
}

fn pt1(input: Vec<Value>) -> Result<Value> {
    let (mut input_queues, mut nics) = init_network(&input);
    let mut result = None;
    loop {
        let mut all_halted = true;
        for (self_addr, nic) in nics.iter_mut().enumerate() {
            let vm = &mut nic.vm;
            let did_yield = &mut nic.did_yield;
            let output_batch = &mut nic.output_batch;
            all_halted &= vm.run_all_async(|io| match io {
                IoOperation::Read(out) => {
                    if result.is_some() {
                        return Ok(());
                    }
                    *out = if let Some(value) = input_queues[self_addr].pop_front() {
                        *did_yield = false;
                        Some(value)
                    } else {
                        if *did_yield {
                            *did_yield = false;
                            Some(-1)
                        } else {
                            *did_yield = true;
                            None
                        }
                    };
                    Ok(())
                }
                IoOperation::Write(value) => {
                    output_batch.push(value);
                    if output_batch.len() == 3 {
                        let (addr, x, y) = (output_batch[0], output_batch[1], output_batch[2]);
                        if addr == 255 {
                            if result.is_none() {
                                result = Some(y);
                            }
                        }
                        if addr >= 0 && addr < 50 {
                            input_queues[addr as usize].push_back(x);
                            input_queues[addr as usize].push_back(y);
                        }
                        output_batch.clear();
                    }
                    Ok(())
                }
            })?;
        }
        if result.is_some() || all_halted {
            break;
        }
    }

    result.ok_or(AoCError::NoSolution)
}

fn pt2(input: Vec<Value>) -> Result<Value> {
    let (mut input_queues, mut nics) = init_network(&input);
    let mut nat = (0, 0);
    let mut idle_for = 0;
    let mut last_reinit = None;
    loop {
        let mut all_halted = true;
        let mut did_write = false;
        for (self_addr, nic) in nics.iter_mut().enumerate() {
            let vm = &mut nic.vm;
            let did_yield = &mut nic.did_yield;
            let output_batch = &mut nic.output_batch;
            all_halted &= vm.run_all_async(|io| match io {
                IoOperation::Read(out) => {
                    *out = if let Some(value) = input_queues[self_addr].pop_front() {
                        *did_yield = false;
                        Some(value)
                    } else {
                        if *did_yield {
                            *did_yield = false;
                            Some(-1)
                        } else {
                            *did_yield = true;
                            None
                        }
                    };
                    Ok(())
                }
                IoOperation::Write(value) => {
                    output_batch.push(value);
                    if output_batch.len() == 3 {
                        let (addr, x, y) = (output_batch[0], output_batch[1], output_batch[2]);
                        did_write = true;
                        if addr == 255 {
                            nat = (x, y);
                        }
                        if addr >= 0 && addr < 50 {
                            input_queues[addr as usize].push_back(x);
                            input_queues[addr as usize].push_back(y);
                        }
                        output_batch.clear();
                    }
                    Ok(())
                }
            })?;
        }
        if all_halted {
            return Err(AoCError::Logic("NICs halted"));
        }
        if did_write {
            idle_for = 0;
            continue;
        } else if idle_for == 1 {
            idle_for = 2;
        } else {
            idle_for = 0;
            input_queues[0].push_back(nat.0);
            input_queues[0].push_back(nat.1);
            if let Some(y) = last_reinit {
                if nat.1 == y {
                    return Ok(y);
                }
            }
            last_reinit = Some(nat.1);
        }
    }
}

// fn pt1(input: Vec<Value>) -> Result<Value> {
//     let mut senders = Vec::with_capacity(50);
//     let mut pre_nics = Vec::with_capacity(50);
//     for addr in 0..50 {
//         let (sender, receiver) = mpsc::channel::<Value>();
//         sender.send(addr as Value).unwrap();
//         senders.push(sender);
//         let vm = VM::new(growing_memory(input.clone()));
//         pre_nics.push((receiver, vm));
//     }

//     let mut join_handles = Vec::with_capacity(50);
//     for (receiver, mut vm) in pre_nics {
//         let senders = senders.clone();
//         let result = result.clone();
//         join_handles.push(thread::spawn(move || {
//             vm.run_all(
//                 || {
//                     Ok(match receiver.try_recv() {
//                         Ok(v) => v,
//                         Err(mpsc::TryRecvError::Empty) => {
//                             thread::yield_now();
//                             -1
//                         }
//                         Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
//                     })
//                 },
//                 write_batching(|(target, x, y)| {
//                     if target == 255 {
//                         let mut result = result.lock().unwrap();
//                         if result.is_none() {
//                             *result = Some(y);
//                             println!("\nout: {}", y);
//                         }
//                         return Ok(());
//                     }
//                     if target < 0 || target >= 50 {
//                         return Err(icError::Custom(format!(
//                             "target machine {} doesn't exit",
//                             target
//                         )));
//                     }
//                     let sender = &senders[target as usize];
//                     sender.send(x).unwrap();
//                     sender.send(y).unwrap();
//                     Ok(())
//                 }),
//             )
//             .unwrap();
//         }));
//     }

//     for join_handle in join_handles {
//         join_handle.join().unwrap();
//     }

//     let result = result.lock().unwrap();
//     Ok(result.unwrap())
// }
