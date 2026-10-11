//    .map(|&x| x*x)
//    Map iterates over each element outputting one replacement element each

//    .flat_map(|&x| vec![*x,(*x)+1,(*x)+2])
//    Flat Map iterates over each element outputting potentially multiple elements each, and then flattening it all down into the single dimensional iterator

//    .filter

/*
 *  In Java, looping through elements of a collection is typically done as
 *      for (int i = 0;i < integers.length;i++) {}
 *  In Rust, this syntax changes in its most basic form to eliminate parenthesis and introduce ranges
 *      for i in 0..integers.len() {}
 *  In Java, if we want to loop through each element rather than each index we
 *      for (int i : integers) {}
 *  In Rust, if we want to loop through each element... it's a bit more involved but ignoring nuance
 *      for i in integers {}
 *  Now to understand the nuance, Rust has iterators. Iterators function in three ways: borrow, borrow mutably, and transferring ownership.
 *      borrow: for i in integers.iter() {}
 *          shorthand: for i in &integers {}
 *      borrow mutably: for i in integers.iter_mut() {}
 *          shorthand: for i in &mut integers {}
 *      transfer ownership: for i in integers.into_iter() {}
 *          shorthand: for i in integers {}
 *  Once you have an iterator, these are the common functions that build off iterators
 *      .for_each(): Runs an operation on each element
 *      .map(): Transforms each element in accordance to a function passed within
 *      .filter(): Keeps elements matching a specific condition passed within
 *      .enumerate(): Provides element AND index at each iteration
 *      .find(): Returns the first matching element
 *      .any(): Checks whether AT LEAST one element matches the condition passed in
 *      .collect(): Consumes the iterator and builds a result collection
 */

fn main() {
    // Borrow the playlist; yield read-only references.
    impl<'a> IntoIterator for &'a Playlist {
        type Item = &'a Track;
        type IntoIter = std::slice::Iter<'a, Track>;
        fn into_iter(self) -> Self::IntoIter {
            self.tracks.iter()
        }
    }
    // Mutably borrow the playlist; yield mutable references.
    impl<'a> IntoIterator for &'a mut Playlist {
        type Item = &'a mut Track;
        type IntoIter = std::slice::IterMut<'a, Track>;
        fn into_iter(self) -> Self::IntoIter {
            self.tracks.iter_mut()
        }
    }
    // Consume the playlist; yield owned tracks.
    impl IntoIterator for Playlist {
        type Item = Track;
        type IntoIter = std::vec::IntoIter<Track>;
        fn into_iter(self) -> Self::IntoIter {
            self.tracks.into_iter()
        }
    }

    // Item = &Track
    for track in &playlist {
        println!("{}", track.title);
    }
    // Item = &mut Track
    for track in &mut playlist {
        track.title.push_str(" (edited)");
    }
    // Item = Track
    for track in playlist {
        println!("{}", track.title);
    }
    // playlist has been consumed
}