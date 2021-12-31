VGL is a graphics library that tries to be way more easier to use than raw vulkan. 

# IMPORTANT

THIS PROJECT IS STILL IN DEVELOPMENT. If you want to use a vulkan graphics library that is actually fully featured, Bevy is probably a better fit for you. That said, if ever you want to program in vulkan and understand it without having to bother with boilerplate code, have at it using this repo.  

# Downloading and building

Just go to my snake_game repo and see how it's done. Vgl is just a simple rust library so I don't think you need I need to go over it too much.

# Testing

In this project, I divided tests into two categories : the normal logic ones and the ones that verify that running all the library does not crash (notably with vulkano). One of the major reasons for this is that running vulkano needs to be on the main thread which means no multithreading for all the unit tests. As such, to verify the entirety of the library, use the following command : 

```bash
cargo test -- --ignored --test-threads=1
```

And if you want to run all the other unit tests, just use the default command : 

```bash
cargo test
```
