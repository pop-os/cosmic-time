Very much still a WIP. API is mostly final and this can create animations. Just missing large amounts of features to make this useful in any real project.

TODOs before release:
- [x] No heap allocations in animation render loop
- [x] Compile time type guarentee that animation id will match correct animation to correct widget type.
- [x] Animatable container widget
- [x] Looping animations
- [x] Animation easing
- [x] add space widget
- [ ] add button widget
- [ ] add row widget
- [ ] add column widget
- [ ] add toggle button widget
- [ ] Use iced 0.7
- [ ] use iced 0.7's framerate subscription
- [x] Add logic for different animation Ease values
- [ ] Documentation
- [ ] Add `Cosmic` cargo feature for compatibility with both iced and System76's temporary fork.
- [x] optimize for `as_subscription` logic

Other TODOs:
- [x] test for easing
- [ ] general animation logic tests
- [ ] physics based animations
- [ ] Add pause for looping animations
- [ ] Low motion accesability detection to disable animations.
- [ ] Figure out what else needs to be on this list
- [ ] Work on web via wasm-unknown-unknown builds
