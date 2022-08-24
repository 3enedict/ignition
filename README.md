Ignition is trying to be (one day), a beautifully simple graphics engine. Hopefully....

# TODO
## Refactoring
- Refactor headless
  - Add RendererCommands structs
- Refactor triangle
- Adding back VertexGroups
- Error handling: Calling function in Engine that stops current frame and starts anew

## ECS
- Add entity
- Add component
- Remove component
- Toggle component
 
# Code layout (to help remember what the hell is going on...)
- *lib.rs*: Home of the infamous **Engine** with it's configuration and it's constructors (plural because there are different kinds of Renderers to think about)
  - *prelude.rs*: Syntactic sugar for importing all important types and functions from *ignition*
  - *manifestation.rs*: Definitions for all kinds of renderers and their constructors (currently: Screen, Image, Headless)
    - *lift_off.rs*: List of useful functions for initializing the link to our nice gpu (used in *manifestation.rs*)
    - *nostalgia.rs*: Everything to do with allocating memory on the gpu (think buffers)
    - *painting.rs*: Textures  (may be moved to nostalgia in the future) 
    - *artist.rs*: This is where the actual rendering happens (this particular file groups utilities for dealing with event_loops, which might be moved to a potential *race_track.rs*)
      - *commands.rs*: Record a command buffer (which regroups all the necessary steps that the gpu has to take to render everything)
      - *pipeline.rs*: Create a pipeline (which describes how a particular object should be rendered)