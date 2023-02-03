use iced_futures::subscription::Subscription;
use iced_native::widget;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::keyframes::Repeat;

pub struct Timeline {
    // Hash map of widget::id to track, where each track is made of subtracks<isize>
    tracks: HashMap<widget::Id, (Repeat, Vec<Vec<SubFrame>>)>,
    // Pending keyframes. Need to call `start` to finalize start time and move into `tracks`
    pending: Vec<(widget::Id, Repeat, Vec<Vec<(Duration, isize)>>)>,
}

pub struct Chain<T: ExactSizeIterator<Item = Option<(Duration, isize)>>> {
  pub id: widget::Id,
  pub repeat: Repeat,
  links: Vec<T>,
}

impl<T: ExactSizeIterator<Item = Option<(Duration, isize)>>> Chain<T> {
  pub fn new(id: widget::Id, repeat: Repeat, links: Vec<T>) -> Self {
    Chain {
      id,
      repeat,
      links,
    }
  }
  fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
    self.links.iter_mut()
  }

  fn into_iter(self) -> impl Iterator<Item=T> {
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
          T: ExactSizeIterator<Item = Option<(Duration, isize)>>
    {
        // TODO better performance? Might be better to iter directly into self.tracks without the
        // `track` middle man. Might also be better to use Option<vec> rather than empty `vec`s.
        // Both should only allocate same on stack though.
        // TODO add garbage collection. If there are > 1 keyframes where time is less than now, remove the extras.
        let id = chain.id.clone();
        let repeat = chain.repeat;
        let mut tracks: Vec<Vec<(Duration, isize)>> = Vec::new();
        let mut chain = chain.into_iter();

        if let Some(modifiers) = chain.next() {
          tracks.reserve(modifiers.len());
          for (track, modifier) in tracks.iter_mut().zip(modifiers.into_iter()) {
            if let Some((at, m)) = modifier {
              track.push((at, m));
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
          let tracks: Vec<Vec<SubFrame>> = tracks.iter_mut().map(|track| track.iter_mut().map(|(duration, value)| SubFrame::new(now + *duration, *value)).collect()).collect();
          let _ = self.tracks.insert(id, (repeat, tracks));
        }
    }

    pub fn get(&self, id: &widget::Id, now: &Instant, index: usize) -> Option<isize> {
        let mut subtrack = if let Some(subtrack) = self.tracks.get(id) {
            subtrack.1
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
          // !!! TODO need to check if animation loops
            track.1
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
