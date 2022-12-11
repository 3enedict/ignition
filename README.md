Ignition is trying to be (one day), a beautifully simple graphics engine. Hopefully....

## TODO

- Test if ecs works in external crates
- Add tests for ecs macros
- Add comments to explain all f****** regexes
- Replace check_if_components_locked with file check
- Add function to check components.toml hasn't been tampered with
- Subdivide engine macro into engine!() and ecs!() for tests
- Add engine trait
- Add parameters for engine macro (name, headless: Hydrogen, screen: Diesel, default: Engine) perhaps by parsing it using syn::Punctuated
- Make triangle test work
- Try loading shaders in a more friendly way
- Add back VertexGroups
- Implement the idea behind RendererCommands using ECS
- Add [] operators for Scene
- Divide into workspaces
- Redo code layout

scene.components(|n| generate_entity_from_id(n)).components(...).entities(1000);
 
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