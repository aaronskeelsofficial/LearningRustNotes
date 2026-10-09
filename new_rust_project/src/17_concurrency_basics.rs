//    For concurrency, there are the following primary standards:
//    Rc<>: This allows you to sort of "dissolve" ownership into the clouds such that the value is dropped once all (immutable) references you spawn to it are dropped.
//    Rc<RefCell<>>: This is an Rc<> but adds in mutability to each reference. Checks there is only one mutable borrower at runtime or panics.
//    Arc<Mutex<>>: This uses an Atomic Rc which is an Rc intended for multithreading purposes as it uses atomic operations. Specifically, it bundles "checking and modifying" the
//      reference counter variable into one step, so one thread modifying it does not desync with another thread modifying it because it's impossible for the processor to handle
//      multiple single "instructions" (grouping of instructions) at a time.
//      The Mutex is used to lock an entire struct/object and all its inner fields, blocking all other threads who are waiting to access it. Locking an entire set of data with
//      Mutex is preferred over individual atomic operations as sometimes computations need to be done across multiple variables and atomic operations are limited in complexity
//      and geared towards a single variable's modification. This is preferred if mutability is needed.
//    Arc<RwLock<>>: Similar to an Arc<Mutex<>> but focuses on having unlimited simultaneous immutable references or one singular writing reference at a time without blocking.
//      If you attempt to mix and match reading with writing sporadically, then probably just stick towards Mutex.
//      - PROBLEM: Beware of unfair "starvation". An example of this would be a bunch of threads pulling read locks simultaneously, while a write lock request is currently blocked.
//        It is possible that the read requests keep coming and going such that there is never a blank time where the write lock can process since they aren't queued.
//    parking_lot::RwLock: Offers a std RwLock replacement that is fair (refuses to infinitely starve write/read blocks) with a time-based queue system