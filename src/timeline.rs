use iced_futures::subscription::Subscription;
use iced_native::widget;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::keyframes::Repeat;
use crate::{lerp, Ease, Tween};

#[derive(Debug, Clone)]
pub struct Timeline {
    // Hash map of widget::id to track, where each track is made of subtracks<isize>
    tracks: HashMap<widget::Id, (Meta, Vec<Vec<SubFrame>>)>,
    // Pending keyframes. Need to call `start` to finalize start time and move into `tracks`
    pendings: Vec<Pending>,
    // Global animation interp value. Use `timeline.now(instant)`, where instant is the value
    // passed from the `timeline.as_subscription` value.
    now: Instant,
}

impl std::default::Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Chain<T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug> {
    pub id: widget::Id,
    pub repeat: Repeat,
    links: Vec<T>,
}

impl<T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug> Chain<T> {
    pub fn new(id: widget::Id, repeat: Repeat, links: Vec<T>) -> Self {
        Chain { id, repeat, links }
    }

    fn into_iter(self) -> impl Iterator<Item = T> {
        self.links.into_iter()
    }
}

#[derive(Debug, Clone)]
enum Pending {
    Chain(PendingChain),
    Pause(widget::Id),
    Resume(widget::Id),
    PauseAll,
    ResumeAll,
}

#[derive(Debug, Clone)]
struct PendingChain {
    id: widget::Id,
    repeat: Repeat,
    tracks: Vec<Vec<DurFrame>>,
    pause: Pause,
}

impl PendingChain {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        id: widget::Id,
        repeat: Repeat,
        tracks: Vec<Vec<DurFrame>>,
        pause: Pause,
    ) -> Pending {
        Pending::Chain(PendingChain {
            id,
            repeat,
            tracks,
            pause,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DurFrame {
    duration: Duration,
    ease: Ease,
    value: f32,
}

impl DurFrame {
    pub fn new(duration: Duration, value: f32, ease: Ease) -> Self {
        DurFrame {
            duration,
            value,
            ease,
        }
    }

    pub fn to_subframe(self, now: Instant) -> SubFrame {
        SubFrame::new(now + self.duration, self.value, self.ease)
    }
}

#[derive(Clone, Debug)]
pub struct Meta {
    pub repeat: Repeat,
    pub start: Instant,
    pub end: Instant,
    pub length: Duration,
    pub pause: Pause,
}

impl Meta {
    pub fn new(
        repeat: Repeat,
        start: Instant,
        end: Instant,
        length: Duration,
        pause: Pause,
    ) -> Self {
        Meta {
            repeat,
            start,
            end,
            length,
            pause,
        }
    }

    pub fn pause(&mut self, now: Instant) {
        if let Pause::Resumed(delay) = self.pause {
            self.pause = Pause::Paused(relative_time(&(now - delay), self));
        } else {
            self.pause = Pause::Paused(relative_time(&now, self));
        }
    }

    pub fn resume(&mut self, now: Instant) {
        if let Pause::Paused(start) = self.pause {
            self.pause = Pause::Resumed(now - start);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pause {
    // Currently paused, with the relative instant into the animation it was paused at.
    Paused(Instant),
    // Has never been paused
    NoPause,
    // The animation was paused, but no longer. The duration is required for the
    // offset of the animation.
    Resumed(Duration),
}

impl Pause {
    pub fn is_playing(&self) -> bool {
        !matches!(self, Pause::Paused(_))
    }
}

#[derive(Debug, Clone)]
pub struct SubFrame {
    pub value: f32,
    pub ease: Ease,
    pub at: Instant,
}

// an intermediary type. This lets the timeline easily
// interpolate between keyframes. Keyframe implementations
// shouldn't have to know about this type. The Instant for this
// (and thus the keyframe itself) is applied with `start`
impl SubFrame {
    pub fn new(at: Instant, value: f32, ease: Ease) -> Self {
        SubFrame { value, at, ease }
    }
}

// equal if instants are equal
impl PartialEq for SubFrame {
    fn eq(&self, other: &Self) -> bool {
        self.at == other.at
    }
}

impl Eq for SubFrame {}

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

pub struct Interped {
    pub previous: f32,
    pub next: f32,
    pub value: f32,
    pub percent: f32,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            tracks: HashMap::new(),
            pendings: Vec::new(),
            now: Instant::now(),
        }
    }

    // Does using clear make more sense? Would be more efficent,
    // But potentially a memory leak?
    pub fn remove_pending(&mut self) {
        self.pendings = Vec::new();
    }

    pub fn pause(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        self.pendings.push(Pending::Pause(id));
        self
    }

    pub fn resume(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        self.pendings.push(Pending::Resume(id));
        self
    }

    pub fn pause_all(&mut self) -> &mut Self {
        self.pendings.push(Pending::PauseAll);
        self
    }

    pub fn resume_all(&mut self) -> &mut Self {
        self.pendings.push(Pending::ResumeAll);
        self
    }

    pub fn set_chain<T>(&mut self, chain: impl Into<Chain<T>>) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug,
    {
        self.set_chain_with_options(chain, Pause::NoPause)
    }

    pub fn set_chain_paused<T>(&mut self, chain: impl Into<Chain<T>>) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug,
    {
        self.set_chain_with_options(chain, Pause::Paused(Instant::now()))
    }

    /// Destructure keyframe into subtracks (via impl ExactSizeIterator) and add to timeline.
    fn set_chain_with_options<T>(&mut self, chain: impl Into<Chain<T>>, pause: Pause) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<DurFrame>> + std::fmt::Debug,
    {
        let chain: Chain<T> = chain.into();
        let id = chain.id.clone();
        let repeat = chain.repeat;
        let mut tracks: Vec<Vec<DurFrame>> = Vec::new();
        let mut chain = chain.into_iter();

        if let Some(modifiers) = chain.next() {
            tracks.resize_with(modifiers.len(), Vec::new);
            for (track, modifier) in tracks.iter_mut().zip(modifiers.into_iter()) {
                if let Some(durframe) = modifier {
                    track.push(durframe)
                }
            }
        }

        for modifiers in chain {
            for (track, modifier) in tracks.iter_mut().zip(modifiers.into_iter()) {
                if let Some(durframe) = modifier {
                    track.push(durframe)
                }
            }
        }

        self.pendings
            .push(PendingChain::new(id, repeat, tracks, pause));
        self
    }

    pub fn clear_chain(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.tracks.remove(&id);
        self
    }

    pub fn now(&mut self, now: Instant) {
        self.now = now;
    }

    pub fn start(&mut self) {
        self.start_at(Instant::now());
    }

    pub fn start_at(&mut self, now: Instant) {
        self.now(now);
        for pending in self.pendings.drain(0..) {
            match pending {
                Pending::Chain(PendingChain {
                    id,
                    repeat,
                    mut tracks,
                    pause,
                }) => {
                    let mut end = now;
                    // The time that the chain was `set_chain_paused` is not
                    // necessaritly the same as the atomic pause time used here.
                    // Fix that here.
                    let pause = if let Pause::Paused(_instant) = pause {
                        Pause::Paused(now)
                    } else {
                        pause
                    };

                    let tracks: Vec<Vec<SubFrame>> = tracks
                        .iter_mut()
                        .map(|track| {
                            track
                                .iter_mut()
                                .inspect(|durframe| end = end.max(now + durframe.duration))
                                .map(|durframe| durframe.to_subframe(now))
                                .collect()
                        })
                        .collect();

                    let meta = Meta::new(repeat, now, end, end - now, pause);
                    let _ = self.tracks.insert(id, (meta, tracks));
                }
                Pending::Pause(id) => {
                    if let Some((meta, _track)) = self.tracks.get_mut(&id) {
                        meta.pause(now);
                    }
                }
                Pending::Resume(id) => {
                    if let Some((meta, _track)) = self.tracks.get_mut(&id) {
                        meta.resume(now);
                    }
                }
                Pending::PauseAll => {
                    for (meta, _track) in self.tracks.values_mut() {
                        meta.pause(now);
                    }
                }
                Pending::ResumeAll => {
                    for (meta, _track) in self.tracks.values_mut() {
                        meta.resume(now);
                    }
                }
            }
        }
    }

    pub fn get(&self, id: &widget::Id, index: usize) -> Option<Interped> {
        let now = &self.now;
        // Get requested modifier_timeline or skip
        let (meta, mut modifier_timeline) = if let Some((meta, chain)) = self.tracks.get(id) {
            if let Some(modifier_timeline) = chain.get(index) {
                (meta, modifier_timeline.iter())
            } else {
                return None;
            }
        } else {
            return None;
        };

        let relative_now = match meta.pause {
            Pause::NoPause => relative_time(now, meta),
            Pause::Resumed(delay) => relative_time(&(*now - delay), meta),
            Pause::Paused(time) => relative_time(&time, meta),
        };

        // Loop through modifier_timeline, returning the interpolated value if possible.
        let mut accumulator: Option<&SubFrame> = None;
        loop {
            match (accumulator, modifier_timeline.next()) {
                (None, Some(modifier)) => {
                    // Found first element in timeline
                    if modifier.at <= relative_now {
                        accumulator = Some(modifier)
                    }
                }
                (None, None) => return None, // No elements in timeline
                (Some(acc), None) => {
                    // Accumulator found in previous loop, but no greater value. Means animation duration has expired.
                    return Some(Interped {
                        previous: acc.value,
                        next: acc.value,
                        percent: 1.0,
                        value: acc.value,
                    });
                }
                (Some(acc), Some(modifier)) => {
                    // Found accumulator in middle-ish of timeline
                    if relative_now >= modifier.at || acc.value == modifier.value {
                        // Can not interpolate between this one and next value?
                        accumulator = Some(modifier);
                    } else {
                        // Can interpolate between these two, thus calculate and return that value.
                        let elapsed = relative_now.duration_since(acc.at).as_millis() as f32;
                        let duration = (modifier.at - acc.at).as_millis() as f32;

                        let previous = acc.value;
                        let next = modifier.value;
                        let percent = modifier.ease.tween(elapsed / duration);
                        let value = lerp(
                            acc.value,
                            modifier.value,
                            modifier.ease.tween(elapsed / duration),
                        );

                        return Some(Interped {
                            previous,
                            next,
                            percent,
                            value,
                        });
                    }
                }
            }
        }
    }

    pub fn as_subscription<Event>(
        &self,
    ) -> Subscription<iced_native::Hasher, (iced_native::Event, iced_native::event::Status), Instant>
    {
        let now = self.now;
        if self.tracks.values().any(|track| {
            (track.0.repeat == Repeat::Forever && track.0.pause.is_playing())
                || (track.0.end >= now && track.0.pause.is_playing())
        }) {
            iced::window::frames()
        } else {
            Subscription::none()
        }
    }
}

// Used for animations that loop.
// Given the current `Instant`, it returns the relative instant in the animation that
// corresponds with the first loop of the animation.
fn relative_time(now: &Instant, meta: &Meta) -> Instant {
    if meta.repeat == Repeat::Never {
        *now
    } else {
        let repeat_num = (*now - meta.start).as_millis() / meta.length.as_millis();
        let reduce_by = repeat_num * meta.length.as_millis();
        now.checked_sub(Duration::from_millis(
            reduce_by.clamp(0, u64::MAX.into()).try_into().unwrap(),
        ))
        .expect("Your animatiion has been runnning for 5.84x10^6 centuries.")
    }
}
