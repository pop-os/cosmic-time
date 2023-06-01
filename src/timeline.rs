#[cfg(feature = "libcosmic")]
mod imports {
    pub use cosmic::iced::time::{Duration, Instant};
    pub use cosmic::iced_core::{
        event::{self, Event},
        widget, Hasher,
    };
    pub use cosmic::iced_futures::subscription::Subscription;
}
#[cfg(not(feature = "libcosmic"))]
mod imports {
    pub use iced::time::{Duration, Instant};
    pub use iced_futures::subscription::Subscription;
    pub use iced_native::{event, widget, Event, Hasher};
}

use imports::{widget, Duration, Instant, Subscription};

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::keyframes::Repeat;
use crate::{lerp, Ease, MovementType, Tween};

/// This holds all the data for your animations.
/// tracks: this holds all data for active animations
/// pendings: this holds all data that hasn't been `.start()`ed yet.
/// now: This is the instant that is used to calculate animation interpolations.
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

/// All "keyframes" have their own chain to make the API friendly.
/// But all chain types need to `impl Into<>` this chain type, so that
/// the [`Timeline`] can hold and manipulate that data.
#[derive(Debug, Clone)]
pub struct Chain {
    /// The Id that refers to this animation. Same Id type that Iced uses.
    pub id: widget::Id,
    /// Should we loop this animation? This field decides that.
    pub repeat: Repeat,
    links: Vec<Vec<Option<Frame>>>,
}

impl Chain {
    /// Create a new chain.
    pub fn new(id: widget::Id, repeat: Repeat, links: impl Into<Vec<Vec<Option<Frame>>>>) -> Self {
        let links = links.into();
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

/// A Frame is the exact value of the modifier at a given time.
/// Advanced use only.
/// You do not need this type unless you are making your own custom animate-able widget.
/// A `Frame::Eager` refers to a known widget's modifier state. This is for most cases
/// like "animate to width 10".
/// A `Frame::Lazy` is for continueing a previous animation, either midway through
/// the animation, or even after the animation was completed.
#[derive(Debug, Clone, Copy)]
pub enum Frame {
    /// Keyframe time, !!VALUE AT TIME!!, ease type into value
    Eager(MovementType, f32, Ease),
    /// Keyframe time, !!DEFAULT FALLBACK VALUE!!, ease type into value
    Lazy(MovementType, f32, Ease),
}

impl Frame {
    /// Create an Eager Frame.
    pub fn eager(movement_type: impl Into<MovementType>, value: f32, ease: Ease) -> Self {
        let movement_type = movement_type.into();
        Frame::Eager(movement_type, value, ease)
    }

    /// Create an Lazy Frame.
    pub fn lazy(movement_type: impl Into<MovementType>, default: f32, ease: Ease) -> Self {
        let movement_type = movement_type.into();
        Frame::Lazy(movement_type, default, ease)
    }

    /// You almost certainly do not need this function.
    /// Used in timeline::start to guarentee that we have the same
    /// time of an animation, not the API convinient [`MovementType`].
    pub fn to_subframe(self, time: Instant) -> SubFrame {
        let (value, ease) = match self {
            Frame::Eager(_movement_type, value, ease) => (value, ease),
            _ => panic!("Call 'to_eager' first"),
        };

        SubFrame::new(time, value, ease)
    }

    /// You almost certainly do not need this function.
    /// Converts a Lazy [`Frame`] to an Eager [`Frame`].
    pub fn to_eager(&mut self, timeline: &Timeline, id: &widget::Id, index: usize) {
        *self = if let Frame::Lazy(movement_type, default, ease) = *self {
            let value = timeline.get(id, index).map(|i| i.value).unwrap_or(default);
            Frame::Eager(movement_type, value, ease)
        } else {
            *self
        }
    }

    fn get_value(&self) -> f32 {
        match self {
            Frame::Eager(_, value, _) => *value,
            _ => panic!("call 'to_eager' first"),
        }
    }

    /// You almost certainly do not need this function.
    /// Get the duration of a [`Frame`]
    pub fn get_duration(self, previous: &Self) -> Duration {
        match self {
            Frame::Eager(movement_type, value, _ease) => match movement_type {
                MovementType::Duration(duration) => duration,
                MovementType::Speed(speed) => speed.calc_duration(previous.get_value(), value),
            },
            _ => panic!("Call 'to_eager' first"),
        }
    }
}

/// The metadata of an animation. Used by [`Timeline`].
#[derive(Clone, Debug)]
pub struct Meta {
    /// Does the animation repeat? The decides that.
    pub repeat: Repeat,
    /// The specific time the animation started at.
    pub start: Instant,
    /// The time that the animation will end.
    /// Used to optimize [`Timeline::as_subscription`]
    pub end: Instant,
    /// The length of time the animation will last
    pub length: Duration,
    /// Is the animation paused? This decides that.
    pub pause: Pause,
}

impl Meta {
    /// Creates new metadata for an animation.
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

    /// Sets the animation to be paused.
    /// If you are an end user of Cosmic Time, you do not want this.
    /// You want the `pause` function on [`Timeline`].
    pub fn pause(&mut self, now: Instant) {
        if let Pause::Resumed(delay) = self.pause {
            self.pause = Pause::Paused(relative_time(&(now - delay), self));
        } else {
            self.pause = Pause::Paused(relative_time(&now, self));
        }
    }

    /// Sets the animation to be resumed.
    /// If you are an end user of Cosmic Time, you do not want this.
    /// You want the `resume` function on [`Timeline`].
    pub fn resume(&mut self, now: Instant) {
        if let Pause::Paused(start) = self.pause {
            self.pause = Pause::Resumed(now - start);
        }
    }
}

/// A type to help guarentee that a paused animation has the correct data
/// to be resumed and/or continue animating.
#[derive(Debug, Clone, Copy)]
pub enum Pause {
    /// Currently paused, with the relative instant into the animation it was paused at.
    Paused(Instant),
    /// Has never been paused
    NoPause,
    /// The animation was paused, but no longer. The duration is required for the
    /// offset of the animation.
    Resumed(Duration),
}

impl Pause {
    /// A conviniece function to check if an animation is playing.
    pub fn is_playing(&self) -> bool {
        !matches!(self, Pause::Paused(_))
    }
}

/// A Cosmic Time internal type to make animation interpolation
/// calculations more efficient.
/// an intermediary type. This lets the timeline easily
/// interpolate between keyframes. Keyframe implementations
/// shouldn't have to know about this type. The Instant for this
/// (and thus the keyframe itself) is applied with `start`
#[derive(Debug, Clone)]
pub struct SubFrame {
    /// The value, same as a [`Frame`]
    pub value: f32,
    /// The ease used to interpolate into this.
    pub ease: Ease,
    /// The Instant of this. Converted from duration in [`Frame`]
    pub at: Instant,
}

impl SubFrame {
    /// Creates a new SubFrame.
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

/// Returned from [`Timeline::get`]
/// Has all the data needed for simple animtions,
/// and for style-like animations where some data
/// is held in the widget, and not passed to [`Timeline`].
#[derive(Debug, Clone, Copy)]
pub struct Interped {
    /// The previous ['Frame']'s value
    pub previous: f32,
    /// The nexy ['Frame']'s value
    pub next: f32,
    /// The interpolated value.
    pub value: f32,
    /// The percent done of this link in the chain.
    pub percent: f32,
}

impl Timeline {
    /// Creates a new [`Timeline`]. If you don't find this function you are going
    /// to have a bad time.
    pub fn new() -> Self {
        Timeline {
            tracks: HashMap::new(),
            pendings: HashMap::new(),
            now: None,
        }
    }

    /// If you accidently manage to `set_chain`, but then decide to undo that.
    /// If you need this there is probably a better way to re-write your code.
    pub fn remove_pending(&mut self) {
        self.pendings.clear();
    }

    fn get_now(&self) -> Instant {
        match self.now {
            Some(now) => now,
            None => Instant::now(),
        }
    }

    /// Need to pause an animation? Use this! Pass the same widget Id
    /// used to create the chain.
    pub fn pause(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.pendings.insert(id, Pending::Pause);
        self
    }

    /// Need to resume an animation? Use this! Pass the same widget Id
    /// used to pause the chain.
    pub fn resume(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.pendings.insert(id, Pending::Resume);
        self
    }

    /// Hammer Time? Pause all animations with this.
    pub fn pause_all(&mut self) -> &mut Self {
        let _ = self
            .pendings
            .insert(widget::Id::unique(), Pending::PauseAll);
        self
    }

    /// Resume all animations.
    pub fn resume_all(&mut self) -> &mut Self {
        let _ = self
            .pendings
            .insert(widget::Id::unique(), Pending::ResumeAll);
        self
    }

    /// Add an animation chain to the timeline!
    /// Each animation Id is unique. It is imposible to use the same Id
    /// for two animations.
    pub fn set_chain(&mut self, chain: impl Into<Chain>) -> &mut Self {
        self.set_chain_with_options(chain, Pause::NoPause)
    }

    /// Like `set_chain` but the animation will start paused on it's first frame.
    pub fn set_chain_paused(&mut self, chain: impl Into<Chain>) -> &mut Self {
        self.set_chain_with_options(chain, Pause::Paused(Instant::now()))
    }

    fn set_chain_with_options(&mut self, chain: impl Into<Chain>, pause: Pause) -> &mut Self {
        // TODO should be removed. Used iterators for pre-release
        // cosmic-time implementation. Keyframes should just pass a Vec<Vec<Frame>>
        let chain = chain.into();
        let id = chain.id;
        let repeat = chain.repeat;

        let _ = self
            .pendings
            .insert(id, Pending::Chain(repeat, chain.links, pause));
        self
    }

    /// Remove's any animation. Usually not necessary, unless you may have
    /// a very large animation that needs to be "garage collected" when done.
    pub fn clear_chain(&mut self, id: impl Into<widget::Id>) -> &mut Self {
        let id = id.into();
        let _ = self.tracks.remove(&id);
        self
    }

    /// Use this in your `update()`.
    /// Updates the timeline's time so that animations can continue atomically.
    pub fn now(&mut self, now: Instant) {
        self.now = Some(now);
    }

    /// Starts all pending animations.
    pub fn start(&mut self) {
        self.start_at(Instant::now());
    }

    /// Starts all pending animations at some other time that isn't now.
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

    /// Get the [`Interped`] value for an animation.
    /// Use internaly by Cosmic Time.
    /// index is the index that the keyframe arbitratily assigns to each
    /// widget modifier (think width/height).
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

    /// Efficiently request redraws for animations.
    /// Automatically checks if animations are in a state where redraws arn't necessary.
    #[cfg(not(feature = "libcosmic"))]
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

    /// Efficiently request redraws for animations.
    /// Automatically checks if animations are in a state where redraws arn't necessary.
    #[cfg(feature = "libcosmic")]
    pub fn as_subscription(&self) -> Subscription<Instant> {
        let now = self.now;
        if now.is_some()
            && self.tracks.values().any(|track| {
                (track.0.repeat == Repeat::Forever && track.0.pause.is_playing())
                    || (track.0.end >= now.unwrap() && track.0.pause.is_playing())
            })
        {
            cosmic::iced_runtime::window::frames()
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
