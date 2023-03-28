# COSMIC TIME
## An animation toolkit for Iced-rs/Iced

> This Project was build for [Cosmic DE](https://github.com/pop-os/cosmic-epoch). Though this will work for any project that depends on [Iced](https://github.com/iced-rs/iced).


 The goal of this project is to provide a simple API to build and show
 complex animations efficiently in applications built with Iced-rs/Iced.

## Project Goals:
* Full compatibility with Iced and The Elm Architecture.
* Ease of use.
* No math required for any animation.
* No heap allocations in render loop.
* Provide additional animatable widgets.
* Custom widget support (create your own!).

## Overview
To wire cosmic-time into Iced there are five steps to do.

1. Create a [`Timeline`] This is the type that controls the animations.
```rust
struct Counter {
      timeline: Timeline
}

// ~ SNIP

impl Application for Counter {
    // ~ SNIP
     fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { timeline: Timeline::new()}, Command::none())
     }
}
```
2. Add at least one animation to your timeline. This can be done in your
   Application's `new()` or `update()`, or both!
```rust
static CONTAINER: Lazy<id::Container> = Lazy::new(id::Container::unique);

let animation = chain![
  CONTAINER,
  container(Duration::ZERO).width(10),
  container(Duration::from_secs(10)).width(100)
];
self.timeline.set_chain(animation).start();

```
There are some different things here!
  > static CONTAINER: Lazy<id::Container> = Lazy::new(id::Container::unique);

  Cosmic Time refers to each animation with an Id. We export our own, but they are
  Identical to the widget Id's Iced uses for widget operations.
  Each animatable widget needs an Id. And each Id can only refer to one animation.

  > let animation = chain![

  Cosmic Time refers to animations as [`Chain`]s because of how we build then.
  Each [`Keyframe`] is linked together like a chain. The Cosmic Time API doesn't
  say "change your width from 10 to 100". We define each state we want the
  widget to have `.width(10)` at `Duration::ZERO` then `.width(100)` at
  `Duration::from_secs(10)`. Where the `Duration` is the time after the previous
  [`keyframe`]. This is why we call the animations chains. We cannot get to the
  next state without animating though all previous [`Keyframe`]s.

  > self.timeline.set_chain(animation).start();

  Then we need to add the animation to the [`Timeline`]. We call this `.set_chain`,
  because there can only be one chain per Id.
  If we `set_chain` with a different animation with the same Id, the first one is
  replaced. This a actually a feature not a bug!
  As well you can set multiple animations at once:
  `self.timeline.set_chain(animation1).set_chain(animation2).start()`

  > .start()

  This one function call is important enough that we should look at it specifically.
  Cosmic Time is atomic, given the animation state held in the [`Timeline`] at any
  given time the global animations will be the exact same. The value used to 
  calculate any animation's interpolation is global. And we use `.start()` to
  sync them together.
  Say you have two 5 seconds animations running at the same time. They should end
  at the same time right? That all depends on when the widget thinks it's animation
  should start. `.start()` tells all pending animations to start at the moment that
  `.start()` is called. This guarantees they stay in sync.
  IMPORTANT! Be sure to only call `.start()` once per call to `update()`.
  The below is incorrect!
  ```rust
  self.timeline.set_chain(animation1).start();
  self.timeline.set_chain(animation2).start();
  ```
  That code will compile, but will result in the animations not being in sync.

3. Add the Cosmic time Subscription
```rust
  fn subscription(&self) -> Subscription<Message> {
       self.timeline.as_subscription::<Event>().map(Message::Tick)
   }
```

4. Map the subscription to update the timeline's state:
```rust
fn update(&mut self, message: Message) -> Command<Message> {
       match message {
           Message::Tick(now) => self.timeline.now(now),
       }
   }
```
  If you skip this step your animations will not progress!

5. Show the widget in your `view()`!
```rust
anim!(CONTIANER, &self.timeline, contents)
```

All done!
There is a bit of wiring to get Cosmic Time working, but after that it's only
a few lines to create rather complex animations!
See the Pong example to see how a full game of pong can be implemented in
only a few lines!

Done:
- [x] No heap allocations in animation render loop
- [x] Compile time type guarantee that animation id will match correct animation to correct widget type.
- [x] Animatable container widget
- [x] Looping animations
- [x] Animation easing
- [x] test for easing
- [x] add space widget
- [x] add button widget
- [x] add row widget
- [x] add column widget
- [x] add toggle button widget
- [x] Use iced 0.8
- [x] use iced 0.8's framerate subscription
- [x] Add logic for different animation Ease values
- [x] Documentation
- [x] optimize for `as_subscription` logic
- [x] Add pause for animations
- [x] Lazy keyframes. (Keyframes that can use the position of a previous (active or not) animation to start another animation.)

TODOs:
- [ ] Add container and space animiations between arbitraty length units.
      Example: Length::Shrink to Length::Fixed(10.) and/or Length::Fill to Length::Shrink
      Only fixed Length::Fixed(_) are supported currently.
- [ ] Add `Cosmic` cargo feature for compatibility with both iced and System76's temporary fork.
- [ ] Low motion accesability detection to disable animations.
- [ ] general animation logic tests
- [ ] Work on web via wasm-unknown-unknown builds
- [ ] physics based animations
- [ ] Figure out what else needs to be on this list
