use hacspec_riot_runqueue::{
    runqueue_add, runqueue_advance, runqueue_del, runqueue_get_next, runqueue_new,
    RunQueue as HacspecRunQueue, RunqueueId as HacspecRunqueueId, ThreadId as HacspecThreadId,
};

pub type RunqueueId = u8;
pub type ThreadId = u8;

pub struct RunQueue<const N_QUEUES: usize, const N_THREADS: usize> {
    hsrq: HacspecRunQueue,
}

impl<const N_QUEUES: usize, const N_THREADS: usize> RunQueue<{ N_QUEUES }, { N_THREADS }> {
    pub fn new() -> RunQueue<{ N_QUEUES }, { N_THREADS }> {
        RunQueue {
            hsrq: runqueue_new(),
        }
    }

    /// add thread with pid n to runqueue number rq
    pub fn add(&mut self, n: ThreadId, rq: RunqueueId) {
        self.hsrq = runqueue_add(self.hsrq, HacspecThreadId(n), HacspecRunqueueId(rq));
    }

    /// remove thread with pid n from runqueue number rq
    /// @note: this implementation fails if "n" is not the queue's head.
    /// This is fine, RIOT-rs only ever calls del() for the current thread.
    pub fn del(&mut self, n: ThreadId, rq: RunqueueId) {
        self.hsrq = runqueue_del(self.hsrq, HacspecThreadId(n), HacspecRunqueueId(rq));
    }

    /// get pid that should run next
    /// returns the next runnable thread of
    /// the runqueue with the highest index
    pub fn get_next(&self) -> Option<u8> {
        runqueue_get_next(&self.hsrq)
    }

    /// advance runqueue number rq
    /// (this is used to "yield" to another thread of *the same* priority)
    pub fn advance(&mut self, rq: RunqueueId) {
        self.hsrq = runqueue_advance(self.hsrq, HacspecRunqueueId(rq));
    }
}
