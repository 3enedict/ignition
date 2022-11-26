Ignition is trying to be (one day), a beautifully simple graphics engine. Hopefully....

## TODO

- Make components macro a derive macro
- Fix update_components to not add the engine macro as a component
- Fix replace_components_in_file to not overwrite a preexisting crate's components
- Make ecs work in external crate
- Subdivide engine macro into engine!() and ecs!()
- Add parameters for engine macro (name, headless: Hydrogen, screen: Diesel, default: Engine)
- Make triangle test work
- Try loading shaders in a more friendly way
- Add back VertexGroups
- Implement the idea behind RendererCommands using ECS
- Add [] operators for Scene
- Divide into workspaces
- Redo code layout
 
## Code layout (**deprecated**)
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