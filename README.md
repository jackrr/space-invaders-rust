# Space Invaders

Partial implementation of space invaders in Rust with GTK.

Struggled to figure out how to pass around game state (`Game` struct
instance) to GTK callback functions. Made the call to instead use a
message passing pattern to process changes triggered within the
callbacks - at this point enemy moving and keyboard interactions.

## Known issues

- Assets assume local filepaths on my host machine. Should figure out
  the "right" way to manage image assets in this toolchain.

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
