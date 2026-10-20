// Independent harness: these paths compile current production files unchanged.
pub mod resources {
    #[path = "D:/game/tiny-glade/crates/country-core/src/resources/audio_instance.rs"]
    pub mod audio_instance;
    #[path = "D:/game/tiny-glade/crates/country-core/src/resources/audio_resources.rs"]
    pub mod audio_resources;
}
pub mod utils {
    #[path = "D:/game/tiny-glade/crates/country-core/src/utils/audio_utils.rs"]
    pub mod audio_utils;
}

#[cfg(test)]
mod independent {
    use super::resources::audio_resources::AudioSamplesLoadedPromise;
    use std::{sync::{mpsc, Arc, Barrier}, thread, time::Duration};

    #[test]
    fn completion_survives_producer_drop_for_128_separate_owners() {
        let mut promises = Vec::new();
        for id in 0..128 {
            let (sender, promise) = AudioSamplesLoadedPromise::channel();
            thread::spawn(move || sender.send(id).unwrap()).join().unwrap();
            promises.push(promise);
        }
        thread::sleep(Duration::from_millis(1250));
        for (id, promise) in promises.into_iter().enumerate().rev() {
            assert_eq!(promise.recv(), Some(id));
            assert_eq!(promise.recv(), None);
        }
    }

    #[test]
    fn slow_producer_wakes_16_receivers_after_1250_ms() {
        let barrier = Arc::new(Barrier::new(17));
        let mut senders = Vec::new();
        let mut receivers = Vec::new();
        for id in 0..16 {
            let (sender, promise) = AudioSamplesLoadedPromise::channel();
            let barrier = barrier.clone();
            senders.push(sender);
            receivers.push(thread::spawn(move || {
                barrier.wait();
                assert_eq!(promise.recv(), Some(id));
                assert_eq!(promise.recv(), None);
            }));
        }
        barrier.wait();
        thread::sleep(Duration::from_millis(1250));
        assert!(receivers.iter().all(|receiver| !receiver.is_finished()));
        for (id, sender) in senders.into_iter().enumerate() { sender.send(id).unwrap(); }
        for receiver in receivers { receiver.join().unwrap(); }
    }

    #[test]
    fn cancelled_consumers_and_failed_producers_consume_ownership_exactly_once() {
        for id in 0..128 {
            let (sender, promise) = AudioSamplesLoadedPromise::channel();
            drop(promise);
            assert_eq!(sender.send(id).unwrap_err().0, id);
            let (sender, promise) = AudioSamplesLoadedPromise::<usize>::channel();
            drop(sender);
            assert!(std::panic::catch_unwind(|| promise.recv()).is_err());
            assert_eq!(promise.recv(), None);
        }
    }

    #[test]
    fn receiver_option_is_taken_before_blocking() {
        let (sender, promise) = AudioSamplesLoadedPromise::channel();
        let promise = Arc::new(promise);
        let other = promise.clone();
        let receiver = thread::spawn(move || other.recv());
        // Polling a second recv is safe: whichever thread takes the original
        // receiver must be the only blocking consumer. Deliver from a third
        // thread to avoid assuming the scheduler selected either contender.
        let (results, received) = mpsc::channel();
        let second = thread::spawn(move || results.send(promise.recv()).unwrap());
        thread::sleep(Duration::from_millis(20));
        sender.send(13).unwrap();
        let first = receiver.join().unwrap();
        second.join().unwrap();
        let second = received.recv().unwrap();
        assert!(matches!((first, second), (Some(13), None) | (None, Some(13))));
    }
}
