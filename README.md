##  Code to copy for usage

```
#[cfg_attr(feature = "hot", bevy_simple_subsecond_system::prelude::hot)]
```

## Ball types

- Multiplier
Multiplies an impact (explosion, arrow, bouncy) on break
- Explosion
Explodes
- Normal
just kinda breaks
- Bouncy
crashes into other balls and breaks at the end of the turn
- Gravity
pulls balls close to it
- Absorber Ball
A bouncy ball that will absorb the other balls' capabilities on impact


## TODOs
- Animate normal despawn (with sounds)
- Animate backdrop on level start and finish
- Animate walls during the screen transition period
- Animate spheres upon entering a level
