use iced_futures::subscription::Subscription;
use iced_native::widget;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::keyframes::Repeat;
use crate::{lerp, Ease, MovementType, Tween};

#[derive(Debug, Clone)]
pub struct Timeline {
    // Hash map of widget::id to track, where each track is made of subtracks<isize>
    tracks: HashMap<widget::Id, (Meta, Vec<Vec<SubFrame>>)>,
    // Pending keyframes. Need to call `start` to finalize start time and move into `tracks`
    pendings: HashMap<widget::Id, Pending>,
    // Global animation interp value. Use `timeline.now(instant)`, where instant is the value
    // passed from the `timeline.as_subscription` value.
    now: Option<Instant>,
}

impl std::default::Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Chain<T: ExactSizeIterator<Item = Option<Frame>> + std::fmt::Debug> {
    pub id: widget::Id,
    pub repeat: Repeat,
    links: Vec<T>,
}

impl<T: ExactSizeIterator<Item = Option<Frame>> + std::fmt::Debug> Chain<T> {
    pub fn new(id: widget::Id, repeat: Repeat, links: Vec<T>) -> Self {
        Chain { id, repeat, links }
    }
}

#[derive(Debug, Clone)]
enum Pending {
    Chain(Repeat, Vec<Vec<Option<Frame>>>, Pause),
    Pause,
    Resume,
    PauseAll,
    ResumeAll,
}

#[derive(Debug, Clone, Copy)]
pub enum Frame {
    // Keyframe time, value at time, ease type into value
    Eager(usize, MovementType, f32, Ease),
    // Keyframe time, DEFAULT FALLBACK VALUE, ease type into value
    Lazy(usize, MovementType, f32, Ease),
}

impl Frame {
    pub fn eager(
        index: usize,
        movement_type: impl Into<MovementType>,
        value: f32,
        ease: Ease,
    ) -> Self {
        let movement_type = movement_type.into();
        Frame::Eager(index, movement_type, value, ease)
    }

    pub fn lazy(
        index: usize,
        movement_type: impl Into<MovementType>,
        default: f32,
        ease: Ease,
    ) -> Self {
        let movement_type = movement_type.into();
        Frame::Lazy(index, movement_type, default, ease)
    }

    pub fn get_chain_index(&self) -> usize {
        match self {
            Frame::Eager(index, _movement_type, _value, _ease) => *index,
            Frame::Lazy(index, _movement_type, _value, _ease) => *index,
        }
    }

    pub fn to_subframe(self, time: Instant) -> SubFrame {
        let (value, ease) = match self {
            Frame::Eager(_index, _movement_type, value, ease) => (value, ease),
            _ => panic!("Call 'to_eager' first"),
        };

        SubFrame::new(time, value, ease)
    }

    pub fn to_eager(&mut self, timeline: &Timeline, id: &widget::Id, timeline_index: usize) {
        *self = if let Frame::Lazy(chain_index, movement_type, default, ease) = *self {
            let value = timeline
                .get(id, timeline_index)
                .map(|i| i.value)
                .unwrap_or(default);
            Frame::Eager(chain_index, movement_type, value, ease)
        } else {
            *self
        }
    }

    fn get_value(&self) -> f32 {
        match self {
            Frame::Eager(_, _, value, _) => *value,
            _ => panic!("call 'to_eager' first"),
        }
    }

    pub fn get_duration(self, previous: &Self) -> Duration {
        match self {
            Frame::Eager(_index, movement_type, value, _ease) => match movement_type {
                MovementType::Duration(duration) => duration,
                MovementType::Speed(speed) => speed.calc_duration(previous.get_value(), value),
            },
            _ => panic!("Call 'to_eager' first"),
        }
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

#[derive(Debug, Clone, Copy)]
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
            pendings: HashMap::new(),
            now: None,
        }
    }

    // Does using clear make more sense? Would be more efficent,
    // But potentially a memory leak?
    pub fn remove_pending(&mut self) {
        self.pendings.clear();
    }

    fn get_now(&self) -> Instant {
        match self.now {
            Some(now) => now,
            None => Instant::now(),
        }
    }

    pub fn pause(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.pendings.insert(id, Pending::Pause);
        self
    }

    pub fn resume(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.pendings.insert(id, Pending::Resume);
        self
    }

    pub fn pause_all(&mut self) -> &mut Self {
        let _ = self
            .pendings
            .insert(widget::Id::unique(), Pending::PauseAll);
        self
    }

    pub fn resume_all(&mut self) -> &mut Self {
        let _ = self
            .pendings
            .insert(widget::Id::unique(), Pending::ResumeAll);
        self
    }

    pub fn set_chain<T>(&mut self, chain: impl Into<Chain<T>>) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<Frame>> + std::fmt::Debug,
    {
        self.set_chain_with_options(chain, Pause::NoPause)
    }

    pub fn set_chain_paused<T>(&mut self, chain: impl Into<Chain<T>>) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<Frame>> + std::fmt::Debug,
    {
        self.set_chain_with_options(chain, Pause::Paused(Instant::now()))
    }

    /// Destructure keyframe into subtracks (via impl ExactSizeIterator) and add to timeline.
    fn set_chain_with_options<T>(&mut self, chain: impl Into<Chain<T>>, pause: Pause) -> &mut Self
    where
        T: ExactSizeIterator<Item = Option<Frame>> + std::fmt::Debug,
    {
        // TODO should be removed. Used iterators for pre-release
        // cosmic-time implementation. Keyframes should just pass a Vec<Vec<Frame>>
        let chain = chain.into();
        let id = chain.id;
        let repeat = chain.repeat;
        let chain: Vec<Vec<Option<Frame>>> = chain.links.into_iter().map(|m| m.collect()).collect();

        let _ = self
            .pendings
            .insert(id, Pending::Chain(repeat, chain, pause));
        self
    }

    pub fn clear_chain(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.tracks.remove(&id);
        self
    }

    pub fn now(&mut self, now: Instant) {
        self.now = Some(now);
    }

    pub fn start(&mut self) {
        self.start_at(Instant::now());
    }

    pub fn start_at(&mut self, now: Instant) {
        let mut pendings = std::mem::take(&mut self.pendings);
        for (id, pending) in pendings.drain() {
            match pending {
                Pending::Chain(repeat, chain, pause) => {
                    let mut end = now;
                    // The time that the chain was `set_chain_paused` is not
                    // necessaritly the same as the atomic pause time used here.
                    // Fix that here.
                    let pause = if let Pause::Paused(_instant) = pause {
                        Pause::Paused(now)
                    } else {
                        pause
                    };

                    let cols = chain[0].len();
                    let rows = chain.len();
                    let mut peekable = chain.into_iter().peekable();
                    let mut specific_chain = Vec::with_capacity(rows);
                    while let Some(current) = peekable.next() {
                        let time = end;
                        if let Some(next) = peekable.peek() {
                            let mut counter = 0;
                            if let Some((c_frame, n_frame)) =
                                current.iter().zip(next.iter()).find(|(c_frame, n_frame)| {
                                    counter += 1;
                                    c_frame.is_some() && n_frame.is_some()
                                })
                            {
                                let mut c = c_frame.expect("Previous check guarentees saftey");
                                let mut n = n_frame.expect("Previous check guarentees saftey");
                                c.to_eager(self, &id, counter - 1);
                                n.to_eager(self, &id, counter - 1);
                                let duration = n.get_duration(&c);
                                end += duration;
                            }
                        }

                        let specific_row = current.into_iter().enumerate().fold(
                            Vec::with_capacity(cols),
                            |mut acc, (i, maybe_frame)| {
                                if let Some(mut frame) = maybe_frame {
                                    frame.to_eager(self, &id, i);
                                    acc.push(Some(frame.to_subframe(time)))
                                } else {
                                    acc.push(None)
                                }
                                acc
                            },
                        );
                        specific_chain.push(specific_row);
                    }
                    let transposed = specific_chain.into_iter().fold(
                        vec![Vec::new(); cols],
                        |mut acc: Vec<Vec<SubFrame>>, row| {
                            row.into_iter().enumerate().for_each(|(j, maybe_item)| {
                                if let Some(item) = maybe_item {
                                    acc[j].push(item)
                                }
                            });
                            acc
                        },
                    );

                    let meta = Meta::new(repeat, now, end, end - now, pause);
                    let _ = self.tracks.insert(id, (meta, transposed));
                }
                Pending::Pause => {
                    if let Some((meta, _track)) = self.tracks.get_mut(&id) {
                        meta.pause(now);
                    }
                }
                Pending::Resume => {
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
        self.now(now);
    }

    pub fn get(&self, id: &widget::Id, index: usize) -> Option<Interped> {
        let now = self.get_now();
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
            Pause::NoPause => relative_time(&now, meta),
            Pause::Resumed(delay) => relative_time(&(now - delay), meta),
            Pause::Paused(time) => relative_time(&time, meta),
        };

        // Loop through modifier_timeline, returning the interpolated value if possible.
        let mut accumulator: Option<&SubFrame> = None;
        loop {
            match (accumulator, modifier_timeline.next()) {
                // Found first element in timeline
                (None, Some(modifier)) => accumulator = Some(modifier),
                // No Elements in timeline
                (None, None) => return None,
                // Accumulator found in previous loop, but no greater value. Means animation duration has expired.
                (Some(acc), None) => {
                    return Some(Interped {
                        previous: acc.value,
                        next: acc.value,
                        percent: 1.0,
                        value: acc.value,
                    });
                }
                // Found accumulator in middle-ish of timeline
                (Some(acc), Some(modifier)) => {
                    // Can not interpolate between this one and next value?
                    if relative_now >= modifier.at || acc.value == modifier.value {
                        accumulator = Some(modifier);
                    // Can interpolate between these two, thus calculate and return that value.
                    } else {
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
        if now.is_some()
            && self.tracks.values().any(|track| {
                (track.0.repeat == Repeat::Forever && track.0.pause.is_playing())
                    || (track.0.end >= now.unwrap() && track.0.pause.is_playing())
            })
        {
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
