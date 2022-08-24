Ignition is trying to be (one day), a beautifully simple graphics engine. Hopefully....

# TODO
## Refactoring
- Redo directory structure
- More data-driven programming
- Remove need for get_headless_device using config
- Make race track functions easier to read (no generic closures)
- Add back VertexGroups
- Add benches that compare performance with and without ignition
- Error handling: Calling function in Engine that stops current frame and starts anew

## ECS
- Add entity
- Add component
- Remove component
- Toggle component
- Implement the idea behind RendererCommands using ECS
 
# Code layout
- *lib.rs*: Home of the infamous **Engine** with it's configuration and constructors
  - *manifestation.rs*: Definition of the Renderer trait
    - *lift_off.rs*: List of useful utilities for initializing a Renderer
      - *headless.rs*: Headless renderer (run without any output)
      - *screen.rs*: Screen renderer (linked to a window)
      - *image.rs*: Image renderer (linked to an image on the cpu)

    - *artist.rs*: A collection of small functions that aid the rendering process
      - *commands.rs*: Record a command buffer (which regroups all the necessary steps that the gpu has to take to render everything)
      - *pipeline.rs*: Create a pipeline (which describes how a particular object should be rendered)

    - *nostalgia.rs*: Everything to do with allocating memory on the gpu (think buffers or textures)

  - *liberty.rs*: Configuration