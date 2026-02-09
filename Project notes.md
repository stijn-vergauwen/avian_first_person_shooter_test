# Project Notes
Info and documentation for development.


## Project Goals
- Player should control a first-person character, able to walk / run around in the world.
- Should be able to pick up weapons, look at them, aim and shoot.
- Should have an in-game way to configure weapons, their behaviour and stats and stuff like aim position.
- Shooting should feel satisfying and responsive, with recoil, sound, and some visual effects like muzzle flashes and hit marks.
- The in-game world should have interactable elements to test out weapons, like shooting targets and obstacles to move around.


## Code Refactoring & Improvements


## Backlog
- add 'weapon inspector' mode that you can switch to when pressing T
- leave inspector mode on T or esc key
- when inspecting, unlock & show the mouse cursor
- when inspecting, disable player movement and rotation input
- when inspecting, click on object and drag the mouse to rotate
    - rotation should happen on Y axis for horizontal movement, and on X axis for vertical movement
- button to reset object to default orientation (value inside orientation component or quat::identity if component not added)
- when leaving inspector, keep the relative rotation of the object to the player as it had in the inspector
- Should now be able to pick up objects, inspect them, rotate them around, carry them in any orientation, and reset them to their default rotation anytime


## Thoughts For Future Projects
- Reworks for PID controller:
    - make single interface, allows splitting to more specific contrtoller implementations (quaternions, without initial response, etc)
        - I'm thinking a common PidController trait, maybe with a generic T for 'position' type and a associated type for velocity & acceleration that defaults to T
        - then multiple struct variants that implement this trait, a default one that copies what I already have, one for rotation that uses quaternion & scaled axis types, and one that leaves out initial response param so I can remove some of the calculations.
    - make generic components for controlling an entities position or rotation, both directly and through physics engine.
        - Idea is that you spawn in this controller component with it's parameters, and then you only have to interact with it to set the target, updating is handled within the plugin

- Weapon anchor rework idea: it should be an interpolated position between an offset from the player body and an offset from the player head.
    - A possible mismatch that I expect is that sometimes you hold an object that you want to keep holding 'in front of you', sometimes it should move up and down with you and sometimes it shouldn't. I think separating the anchor from being a child element and instead being 'target position offsets' would fix this, you can then interpolate between.