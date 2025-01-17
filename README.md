# Space Invaders

Partial implementation of space invaders in Rust with GTK.

Run with `cargo run`.

## Known issues

- Assets assume local filepaths on my host machine. Should figure out
  the "right" way to manage image assets in this toolchain.
- I don't have the best understanding of the implications of adding
  `Send` to `Game`. Was seemingly necessary to wrap it in a `Mutex`,
  but the `unsafe` has me worried that ... it's unsafe. Something to
  research!

## To NOT do (reserved for later)

- Shooting bullets
- Score keeping
- Collision detection (bullets)
- Enemy life tracking + death

## Future work

- Collision detection self + enemy for game end
- Support window resizing
- Enemy dying animation
- Game over screen w/ new game interaction
