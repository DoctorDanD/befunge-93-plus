
## Befunge-93+

### Wait, doesn't it say "Befunge-93-plus"?

  

Unfortunately, due to technical limitations Github does not allow the character '+' in project titles. However, the correct name is "Befunge-93+"

### Overview

  

This is a toy project I spent a few days on that can interpret and run a befunge file. It is programmed entirely in rust in the hopes that it will be "blazingly fast", but as I am no expert at rust it is surely poorly optimized. However, as far as I can tell it runs pretty darn fast.

  

### Changes

  

This interpreter follows all of the specifications of Befunge-93 (can be found [here](https://esolangs.org/wiki/Befunge)) except for one: the playfield is no longer limited to 80 x 25 cells, and is now near-infinite. However, due to this change ***~~wraparound will no longer work!~~*** It is now working.

  

### Usage

  

Just run the executable in any terminal providing a befunge source file like so:

```

./befunge-93+ [FILE NAME HERE]

```

  

### Releases

  

You can find the executable for windows [here](https://github.com/DoctorDanD/befunge-93-/releases/latest). If there is sufficient demand I will also create executables for other operating systems.
