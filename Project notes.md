# Project Notes
Info and documentation for development.


## Project Goals
- Player should control a first-person character, able to walk / run around in the world.
- Should be able to pick up weapons, look at them, aim and shoot.
- Should have an in-game way to configure weapons, their behaviour and stats and stuff like aim position.
- Shooting should feel satisfying and responsive, with recoil, sound, and some visual effects like muzzle flashes and hit marks.
- The in-game world should have interactable elements to test out weapons, like shooting targets and obstacles to move around.


## Code Refactoring & Improvements
- update item_anchor position etc using forces instead of directly modifying position or rotation.

Weapon anchor rework idea: it should be an interpolated position between an offset from the player body and an offset from the player head. A possible mismatch that I expect is that sometimes you hold an object that you want to keep holding 'in front of you', sometimes it should move up and down with you and sometimes it shouldn't. I think separating the anchor from being a child element and instead being 'target position offsets' would fix this, you can then interpolate between.


## Backlog
- Add test object that can be picked up but isn't a weapon (like a radio)
- Allow items to configure in what orientation they should be held
- Able to shoot weapon that is currently being held


## Thoughts For Future Projects
