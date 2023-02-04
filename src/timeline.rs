use iced_futures::subscription::Subscription;
use iced_native::widget;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::keyframes::Repeat;

pub struct Timeline {
    // Hash map of widget::id to track, where each track is made of subtracks<isize>
    tracks: HashMap<widget::Id, (Meta, Vec<Vec<SubFrame>>)>,
    // Pending keyframes. Need to call `start` to finalize start time and move into `tracks`
    pending: Vec<(widget::Id, Repeat, Vec<Vec<(Duration, isize)>>)>,
}

#[derive(Clone, Debug)]
pub struct Meta {
    pub repeat: Repeat,
    pub start: Instant,
    pub end: Instant,
    pub length: Duration,
}

impl Meta {
    pub fn new(repeat: Repeat, start: Instant, end: Instant, length: Duration) -> Self {
        Meta {
            repeat,
            start,
            end,
            length,
        }
    }
}

pub struct Chain<T: ExactSizeIterator<Item = Option<(Duration, isize)>> + std::fmt::Debug> {
    pub id: widget::Id,
    pub repeat: Repeat,
    links: Vec<T>,
}

impl<T: ExactSizeIterator<Item = Option<(Duration, isize)>> + std::fmt::Debug> Chain<T> {
    pub fn new(id: widget::Id, repeat: Repeat, links: Vec<T>) -> Self {
        Chain { id, repeat, links }
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.links.iter_mut()
    }

    fn into_iter(self) -> impl Iterator<Item = T> {
        self.links.into_iter()
    }
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

    // Does using clear make more sense? Would be more efficent,
    // But potentially a memory leak?
    pub fn remove_pending(&mut self) {
        self.pending = Vec::new();
    }

    /// Destructure keyframe into subtracks (via impl ExactSizeIterator) and add to timeline.
    pub fn set_chain<T>(&mut self, chain: Chain<T>) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<(Duration, isize)>> + std::fmt::Debug,
    {
        let id = chain.id.clone();
        let repeat = chain.repeat;
        let mut tracks: Vec<Vec<(Duration, isize)>> = Vec::new();
        let mut chain = chain.into_iter();

        if let Some(modifiers) = chain.next() {
            tracks.resize_with(modifiers.len(), || Vec::new());
            for (track, modifier) in tracks.iter_mut().zip(modifiers.into_iter()) {
                if let Some((at, m)) = modifier {
                    track.push((at, m))
                }
            }
        }

        for modifiers in chain {
            for (track, modifier) in tracks.iter_mut().zip(modifiers.into_iter()) {
                if let Some((at, m)) = modifier {
                    track.push((at, m))
                }
            }
        }

        self.pending.push((id, repeat, tracks));
        self
    }

    pub fn start(&mut self) {
        self.start_at(Instant::now());
    }

    pub fn start_at(&mut self, now: Instant) {
        for (id, repeat, mut tracks) in self.pending.drain(0..) {
            let mut end = now;
            let tracks: Vec<Vec<SubFrame>> = tracks
                .iter_mut()
                .map(|track| {
                    track
                        .iter_mut()
                        .inspect(|(duration, _)| end = end.max(now + *duration))
                        .map(|(duration, value)| SubFrame::new(now + *duration, *value))
                        .collect()
                })
                .collect();

            let meta = Meta::new(repeat, now, end, end - now);
            let _ = self.tracks.insert(id, (meta, tracks));
        }
    }

    pub fn get(&self, id: &widget::Id, now: &Instant, index: usize) -> Option<isize> {
        let (meta, mut modifier_chain) = if let Some((meta, chain)) = self.tracks.get(id) {
            if let Some(modifier_chain) = chain.get(index) {
                (meta, modifier_chain.iter())
            } else {
                return None;
            }
        } else {
            return None;
        };

        let mut accumulator: Option<&SubFrame> = None;
        loop {
            match (accumulator, modifier_chain.next()) {
                (None, Some(modifier)) => {
                    let relative_now = if meta.repeat == Repeat::Forever {
                        let repeat_num = (*now - meta.start).as_millis() / meta.length.as_millis();
                        let reduce_by = repeat_num * meta.length.as_millis();
                        *now - Duration::from_millis(reduce_by.clamp(0, u64::MAX.into()).try_into().unwrap())
                    } else {
                        *now
                    };
                    if modifier.at <= relative_now { accumulator = Some(modifier) }
                }
                (None, None) => return None,
                (Some(acc), None) => return Some(acc.value),
                (Some(acc), Some(modifier)) => {
                    let relative_now = if meta.repeat == Repeat::Forever {
                        let repeat_num = (*now - meta.start).as_millis() / meta.length.as_millis();
                        let reduce_by = repeat_num * meta.length.as_millis();
                        *now - Duration::from_millis(reduce_by.clamp(0, u64::MAX.into()).try_into().unwrap())
                    } else {
                        *now
                    };
                    if modifier.at <= relative_now {
                        accumulator = Some(modifier);
                    } else if modifier.at >= relative_now {
                        // TODO add different types of interpolations. Likely needs
                        // be an enum stored in SubFrame
                        return Some(calc_linear(&relative_now, acc, modifier));
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
        if self
            .tracks
            .values()
            .any(|track| track.0.repeat == Repeat::Forever || track.0.end >= now)
        {
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
