// See: https://adventofcode.com/2019/day/23
// ## --- Day 23: Category Six ---
//
// The droids have finished repairing as much of the ship as they can. Their report indicates that
// this was a *Category 6* disaster - not because it was that bad, but because it destroyed the
// stockpile of [Category 6][1] network cables as well as most of the ship's network infrastructure.
//
// You'll need to *rebuild the network from scratch*.
//
// The computers on the network are standard [Intcode][2] computers that communicate by sending
// *packets* to each other. There are `50` of them in total, each running a copy of the same
// *Network Interface Controller* (NIC) software (your puzzle input). The computers have *network
// addresses* `0` through `49`; when each computer boots up, it will request its network address via
// a single input instruction. Be sure to give each computer a unique network address.
//
// Once a computer has received its network address, it will begin doing work and communicating over
// the network by sending and receiving *packets*. All packets contain *two values* named `X` and
// `Y`. Packets sent to a computer are queued by the recipient and read in the order they are
// received.
//
// To *send* a packet to another computer, the NIC will use *three output instructions* that provide
// the *destination address* of the packet followed by its `X` and `Y` values. For example, three
// output instructions that provide the values `10`, `20`, `30` would send a packet with `X=20` and
// `Y=30` to the computer with address `10`.
//
// To *receive* a packet from another computer, the NIC will use an *input instruction*. If the
// incoming packet queue is *empty*, provide `-1`. Otherwise, provide the `X` value of the next
// packet; the computer will then use a second input instruction to receive the `Y` value for the
// same packet. Once both values of the packet are read in this way, the packet is removed from the
// queue.
//
// Note that these input and output instructions never [block][3]. Specifically, output instructions
// do not wait for the sent packet to be received - the computer might send multiple packets before
// receiving any. Similarly, input instructions do not wait for a packet to arrive - if no packet is
// waiting, input instructions should receive `-1`.
//
// Boot up all `50` computers and attach them to your network. *What is the `Y` value of the first
// packet sent to address `255`?*
//
// [1] https://en.wikipedia.org/wiki/Category_6_cable
// [2] 9
// [3] https://en.wikipedia.org/wiki/Blocking_(computing)
//
//
// ## --- Part Two ---
//
// Packets sent to address `255` are handled by a device called a NAT (Not Always Transmitting). The
// NAT is responsible for managing power consumption of the network by blocking certain packets and
// watching for idle periods in the computers.
//
// If a packet would be sent to address `255`, the NAT receives it instead. The NAT remembers only
// the *last* packet it receives; that is, the data in each packet it receives overwrites the NAT's
// packet memory with the new packet's `X` and `Y` values.
//
// The NAT also monitors all computers on the network. If all computers have *empty incoming packet
// queues* and are *continuously trying to receive packets* without sending packets, the network is
// considered *idle*.
//
// Once the network is idle, the NAT sends *only the last packet it received* to address `0`; this
// will cause the computers on the network to resume activity. In this way, the NAT can throttle
// power consumption of the network when the ship needs power in other areas.
//
// Monitor packets released to the computer at address `0` by the NAT. *What is the first `Y` value
// delivered by the NAT to the computer at address `0` twice in a row?*

use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

mod _2019;
use _2019::io::{IntRead, IntWrite};
use _2019::{ints_from_str, Int, Program};

lazy_static! {
    static ref PART_1_COMPLETE: AtomicBool = AtomicBool::new(false);
    static ref PART_2_SEEN: Mutex<HashSet<Int>> = Mutex::new(HashSet::new());
}

type NetAddress = usize;

/// Not Always Transmitting
struct NAT {
    pub addr: NetAddress,
    packet: Option<(Int, Int)>,
}

impl NAT {
    pub fn new(addr: NetAddress) -> NAT {
        NAT { addr, packet: None }
    }

    pub fn set_packet(&mut self, packet: (Int, Int)) {
        self.packet = Some(packet);
    }

    pub fn check_idle(&self, network: &mut Network) {
        if let Some((x, y)) = self.packet {
            let is_idle = network
                .iter()
                .all(|(net_addr, buf)| *net_addr == self.addr || buf.is_empty());

            // send resume packet
            if is_idle {
                {
                    let mut set = PART_2_SEEN.lock().unwrap();
                    if set.contains(&y) {
                        aoc_lib::set_part_2!(y);
                        std::process::exit(0);
                    } else {
                        set.insert(y);
                    }
                }

                network.send_int(0, x);
                network.send_int(0, y);
            }
        }
    }
}

struct Network {
    inner: HashMap<NetAddress, Vec<Int>>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            inner: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &Vec<Int>)> {
        self.inner.iter()
    }

    pub fn receive_int(&mut self, net_addr: NetAddress) -> Int {
        let queue = self.inner.entry(net_addr).or_insert(vec![]);
        match queue.len() {
            0 => -1,
            _ => queue.remove(0),
        }
    }

    pub fn receive_packet(&mut self, net_addr: NetAddress) -> Option<(Int, Int)> {
        let queue = self.inner.entry(net_addr).or_insert(vec![]);
        match queue.len() {
            0 | 1 => None,
            _ => Some((queue.remove(0), queue.remove(0))),
        }
    }

    pub fn send_int(&mut self, net_addr: NetAddress, int: Int) {
        let queue = self.inner.entry(net_addr).or_insert(vec![]);
        queue.push(int);
    }
}

#[derive(Clone)]
struct Port {
    addr: usize,
    outgoing: Vec<Int>,
    network: Arc<Mutex<Network>>,
}

impl Port {
    pub fn new(network: Arc<Mutex<Network>>, addr: NetAddress) -> Port {
        Port {
            addr,
            outgoing: vec![],
            network,
        }
    }
}

impl IntRead for Port {
    fn int_read(&mut self) -> Option<Int> {
        let mut inner = self.network.lock().unwrap();
        Some(inner.receive_int(self.addr))
    }
}

impl IntWrite for Port {
    fn int_write(&mut self, int: Int) {
        self.outgoing.push(int);

        // if we've got a full packet, then send it through to the destination program
        if self.outgoing.len() == 3 {
            let y = self.outgoing.pop().unwrap();
            let x = self.outgoing.pop().unwrap();
            let dest_net_addr = self.outgoing.pop().unwrap() as usize;

            if !PART_1_COMPLETE.load(Ordering::SeqCst) && dest_net_addr == 255 {
                aoc_lib::set_part_1!(y);
                PART_1_COMPLETE.store(true, Ordering::SeqCst);
            }

            let mut inner = self.network.lock().unwrap();
            inner.send_int(dest_net_addr, x);
            inner.send_int(dest_net_addr, y);
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("./input/2019-23.txt").trim();
    let ints = ints_from_str(input);

    let net = Arc::new(Mutex::new(Network::new()));
    let mut threads = (0..50)
        .map(|net_addr| {
            let net = net.clone();
            let ints = ints.clone();
            thread::spawn(move || {
                let port = Port::new(net, net_addr);
                let mut program = Program::new(ints);
                // run the program once to give it its network address
                program.run(&mut vec![net_addr as Int], port.clone());
                // now connect it to the network and run it again
                program.run(port.clone(), port);
            })
        })
        .collect::<Vec<JoinHandle<()>>>();

    // setup the NAT
    let mut nat = NAT::new(255);
    threads.push(thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(100));
        {
            let mut net = net.lock().unwrap();
            while let Some(packet) = net.receive_packet(nat.addr) {
                nat.set_packet(packet);
            }
            nat.check_idle(&mut net);
        }
    }));

    // wait for all threads (indefinitely)
    for t in threads {
        let _ = t.join();
    }

    Ok(())
}
