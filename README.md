# Weisbecker

A Chip-8 emulator I built to become more comfortable with Rust and programming virtual machines.

It's still a little bit buggy, but it passes most (not all) common tests for Chip-8 emulators, and can run most roms with varying levels of success.

If you want to try it out, clone this repo and start the emulator by running `cargo run -- <ROM_PATH>`. There are some test roms provided in the `roms` folder of this repo (all of which *are* legal to distribute freely). 

Sometimes, a rom may be buggy or not work very well with a certain tick rate. You can and should try running the emulator with a different tick rate in those cases. By default, it is set to 6, but you can specify a different one by passing it in as an argument like so, `cargo run -- <ROM_PATH> -t 8` (for 8 ticks per frame, as an example).

Also note that some flickering is expected and is accurate. This is an artifact of the sprites being XORed and redrawn repeatedly!

The controls are mapped as shown below:

```
Chip-8        Keyboard
┌─┬─┬─┬─┐     ┌─┬─┬─┬─┐
│1│2│3│C│     │1│2│3│4│
├─┼─┼─┼─┤     ├─┼─┼─┼─┤
│4│5│6│D│     │Q│W│E│R│
├─┼─┼─┼─┤ ==> ├─┼─┼─┼─┤
│7│8│9│E│     │A│S│D│F│
├─┼─┼─┼─┤     ├─┼─┼─┼─┤
│A│0│B│F│     │Z│X│C│V│
└─┴─┴─┴─┘     └─┴─┴─┴─┘
```