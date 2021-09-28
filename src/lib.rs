#![cfg_attr(not(test), no_std)]

/* use Rust version */
#[cfg(not(feature = "hacspec"))]
mod runqueue;

/* use hacspec version */
#[cfg(feature = "hacspec")]
mod hacspec;

#[cfg(feature = "hacspec")]
pub use hacspec::{RunQueue, RunqueueId, ThreadId};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rq_basic() {
        let mut runqueue: RunQueue<8, 32> = RunQueue::new();

        runqueue.add(0, 0);
        runqueue.add(1, 0);
        runqueue.add(2, 0);

        assert_eq!(runqueue.get_next(), Some(0));

        runqueue.advance(0);

        assert_eq!(runqueue.get_next(), Some(1));
        runqueue.advance(0);

        assert_eq!(runqueue.get_next(), Some(2));
        assert_eq!(runqueue.get_next(), Some(2));

        runqueue.advance(0);
        assert_eq!(runqueue.get_next(), Some(0));

        runqueue.advance(0);
        assert_eq!(runqueue.get_next(), Some(1));

        runqueue.advance(0);
        assert_eq!(runqueue.get_next(), Some(2));
    }

    #[test]
    fn test_rq_all20() {
        let mut runqueue: RunQueue<20, 30> = RunQueue::new();

        for i in 0..30 {
            runqueue.add(i, 0);
        }

        for i in 0..30 {
            assert_eq!(runqueue.get_next(), Some(i));
            runqueue.advance(0);
        }

        for i in 0..30 {
            assert_eq!(runqueue.get_next(), Some(i));
            runqueue.advance(0);
        }
    }

    #[test]
    fn test_rq_basic_twoprio() {
        let mut runqueue: RunQueue<8, 32> = RunQueue::new();

        runqueue.add(0, 0);
        runqueue.add(1, 0);
        runqueue.add(3, 0);

        runqueue.add(2, 1);
        runqueue.add(4, 1);

        assert_eq!(runqueue.get_next(), Some(2));
        runqueue.del(2, 1);
        assert_eq!(runqueue.get_next(), Some(4));
        runqueue.del(4, 1);
        assert_eq!(runqueue.get_next(), Some(0));
        runqueue.del(0, 0);
        assert_eq!(runqueue.get_next(), Some(1));
        runqueue.del(1, 0);
        assert_eq!(runqueue.get_next(), Some(3));
        runqueue.del(3, 0);
        assert_eq!(runqueue.get_next(), None);
    }
    #[test]
    fn test_push_twice() {
        let mut runqueue: RunQueue<8, 32> = RunQueue::new();

        runqueue.add(0, 0);
        runqueue.add(1, 0);

        assert_eq!(runqueue.get_next(), Some(0));
        runqueue.del(0, 0);
        assert_eq!(runqueue.get_next(), Some(1));

        runqueue.add(0, 0);

        assert_eq!(runqueue.get_next(), Some(1));

        runqueue.advance(0);
        assert_eq!(runqueue.get_next(), Some(0));
    }
}
