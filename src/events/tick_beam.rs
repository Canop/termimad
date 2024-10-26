use {
    crossbeam::channel::{
        Receiver,
        Sender,
    },
    std::{
        thread,
        time::Duration,
    },
};

pub type TickBeamId = usize;

struct TickBeamHandle {
    id: TickBeamId,
    interrupt_sender: Sender<()>,
}

/// Definition of a beam to be started by a ticker
pub struct TickBeam<P> {
    remaining_count: Option<usize>,
    period: Duration,
    payload: P,
}

/// A tick generator, sending a paylod of your choice (can be `()` for a simple timer)
/// at regular inverval, once, several times or infinitely.
///
/// A simple ticker can handle several beams in parallel, can have them stopped (won't
/// do anything if no beam is active).
///
/// All beams of a ticker generate tick payloads on the same channel, whose receiver
/// can be directly read in this instance.
pub struct Ticker<P> {
    next_id: usize,
    beams: Vec<TickBeamHandle>,
    tick_sender: Sender<P>,
    pub tick_receiver: Receiver<P>,
}

impl TickBeamHandle {
    pub fn stop(self) {
        let _ = self.interrupt_sender.send(());
    }
}

impl<P> Ticker<P> {
    pub fn new() -> Self {
        let (tick_sender, tick_receiver) = crossbeam::channel::unbounded();
        Self {
            next_id: 0,
            beams: Vec::new(),
            tick_sender,
            tick_receiver,
        }
    }
    /// Stop all current beams
    pub fn stop_all_beams(&mut self) {
        for beam in self.beams.drain(..) {
            beam.stop();
        }
    }
    /// Stop (definitely) a beam by its id
    pub fn stop_beam(&mut self, id: TickBeamId) {
        let idx = self.beams.iter().position(|beam| beam.id == id);
        if let Some(idx) = idx {
            self.beams.swap_remove(idx).stop();
        }
    }
}

impl<P: Copy + Send + 'static> Ticker<P> {
    /// Start a new beam, returning its id, wich can be used to stop it
    ///
    /// You may drop the id if you don't plan to manually stop the beam.
    pub fn start_beam(&mut self, mission: TickBeam<P>) -> TickBeamId {
        let id = self.next_id;
        self.next_id += 1;
        let (interrupt_sender, interrupt_receiver) = crossbeam::channel::bounded(1);
        let tick_sender = self.tick_sender.clone();
        thread::spawn(move || {
            let mut remaining_count = mission.remaining_count;
            loop {
                if let Some(remaining_count) = remaining_count.as_mut() {
                    if *remaining_count == 0 {
                        break;
                    }
                    *remaining_count -= 1;
                }
                if interrupt_receiver.recv_timeout(mission.period).is_ok() {
                    // we received an interrupt
                    break;
                }
                let _ = tick_sender.send(mission.payload);
            }
        });
        self.beams.push(TickBeamHandle {
            id,
            interrupt_sender,
        });
        id
    }
    /// Ask for only one tick after a given delay
    pub fn tick_once(&mut self, payload: P, after: Duration) -> TickBeamId {
        self.start_beam(TickBeam {
            remaining_count: Some(1),
            period: after,
            payload,
        })
    }
    /// Require a tick every period, for a given number of times
    ///
    /// The same payload will be sent each time.
    pub fn tick_several_times(&mut self, payload: P, period: Duration, count: usize) -> TickBeamId {
        self.start_beam(TickBeam {
            remaining_count: Some(count),
            period,
            payload,
        })
    }
    /// Require a tick every period, infinitely, with the same payload
    pub fn tick_infinitely(&mut self, payload: P, period: Duration) -> TickBeamId {
        self.start_beam(TickBeam {
            remaining_count: None,
            period,
            payload,
        })
    }
}

impl<P> Drop for Ticker<P> {
    fn drop(&mut self) {
        self.stop_all_beams();
    }
}
