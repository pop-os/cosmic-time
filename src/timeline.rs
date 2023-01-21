use iced_futures::subscription::Subscription;
use iced_native::widget;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::keyframes::IsKeyframe;

pub struct Timeline {
    // Hash map of widget::id to track, where each track is made of subtracks<isize>
    tracks: HashMap<widget::Id, Vec<Vec<SubFrame>>>,
    // Pending keyframes. Need to call `start` to finalize start time and move into `tracks`
    pending: Vec<(widget::Id, Duration, Vec<Vec<isize>>)>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct SubFrame {
    pub value: isize,
    pub at: Instant,
}

// an intermediary type. This lets the timeline easily
// interpolate between keyframes. Keyframe implementations
// shouldn't have to know about this type. The Instant for this
// (and thus the keyframe itself) is applied with `start`
impl SubFrame {
    pub fn new(at: Instant, value: isize) -> Self {
        SubFrame { value, at }
    }
}

// by default sort by time.
impl Ord for SubFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.at.cmp(&other.at)
    }
}

// by default sort by time.
impl PartialOrd for SubFrame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.at.cmp(&other.at))
    }
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            tracks: HashMap::new(),
            pending: Vec::new(),
        }
    }

    /// Destructure keyframe into subtracks (via impl ExactSizeIterator) and add to timeline.
    pub fn add_keyframe<Keyframe>(&mut self, keyframe: Keyframe) -> &mut Self
    where
        Keyframe: IsKeyframe + ExactSizeIterator<Item = Option<isize>>,
    {
        // TODO better performance? Might be better to iter directly into self.tracks without the
        // `track` middle man. Might also be better to use Option<vec> rather than empty `vec`s.
        // Both should only allocate same on stack though.
        // TODO add garbage collection. If there are > 1 keyframes where time is less than now, remove the extras.
        let id = keyframe.id();
        let at = keyframe.at();
        let len = keyframe.len();
        let new_track: Vec<Vec<isize>> =
            keyframe
                .into_iter()
                .fold(Vec::with_capacity(len), |mut acc, modifier| {
                    acc.push(modifier.and_then(|m| Some(vec![m])).unwrap_or(Vec::new()));
                    acc
                });

        self.pending.push((id, at, new_track));
        self
    }

    pub fn start(&mut self) {
        let now = Instant::now();
        for (id, duration, new_track) in self.pending.drain(0..) {
            let is_new = self.tracks.get(&id).is_none();
            if is_new {
                let _ = self.tracks.insert(
                    id,
                    new_track
                        .into_iter()
                        .map(|mut vec| {
                            vec.drain(0..)
                                .map(|value| SubFrame::new(now + duration, value))
                                .collect()
                        })
                        .collect(),
                );
            } else {
                let track = self
                    .tracks
                    .get_mut(&id)
                    .expect("Check above should guarentee existance.");
                track
                    .iter_mut()
                    .zip(new_track.into_iter())
                    .for_each(|(subtrack, mut new)| {
                        if let Some(value) = new.pop() {
                            subtrack.push(SubFrame::new(now + duration, value));
                            subtrack.sort();
                        }
                    });
            }
        }
    }

    pub fn get(&self, id: &widget::Id, now: &Instant, index: usize) -> Option<isize> {
        let mut subtrack = if let Some(subtrack) = self.tracks.get(id) {
            subtrack
                .get(index)
                .expect("proper keyframe implementation should prevent this")
                .iter()
        } else {
            return None;
        };

        let mut accumulator: Option<&SubFrame> = None;
        loop {
            match (accumulator, subtrack.next()) {
                (None, Some(subframe)) => {
                    if &subframe.at <= &now {
                        accumulator = Some(subframe)
                    };
                }
                (None, None) => return None,
                (Some(acc), None) => return Some(acc.value),
                (Some(acc), Some(subframe)) => {
                    if &subframe.at <= &now {
                        accumulator = Some(subframe);
                    } else if &subframe.at >= &now {
                        // TODO add different types of interpolations. Likely needs
                        // be an enum stored in SubFrame
                        return Some(calc_linear(&now, acc, subframe));
                    }
                }
            }
        }
    }

    pub fn as_subscription<H, E>(&self) -> Subscription<H, E, Instant>
    where
        H: std::hash::Hasher,
    {
        let now = Instant::now();
        if self.tracks.values().any(|track| {
            track
                .iter()
                .any(|subtrack| subtrack.iter().any(|subframe| &subframe.at >= &now))
        }) {
            //TODO use iced's new subscription to monitor framerate
            iced::time::every(Duration::from_millis(2))
        } else {
            Subscription::none()
        }
    }
}

// todo should be in module for types of interpolations between points.
fn calc_linear(now: &Instant, lower_bound: &SubFrame, upper_bound: &SubFrame) -> isize {
    let percent_done = (*now - lower_bound.at).as_millis() as f64
        / (upper_bound.at - lower_bound.at).as_millis() as f64;
    let delta = (upper_bound.value - lower_bound.value) as f64;
    let value = (percent_done * delta + (lower_bound.value as f64)) as isize;

    if upper_bound.value > lower_bound.value {
        upper_bound.value.min(value.into())
    } else {
        upper_bound.value.max(value.into())
    }
}
