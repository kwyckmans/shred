# Shred

A simple cli application to help me practice guitar using spaced repetition.

**Disclaimer:** you should probably use something more educationally responsible and/or more polished. 
Half the features in here are better in Anki, and the other half is provided in a neat tool on,
for example, [Justin Guitar](justinguitar.com). The goal of building this was to have something
useful while learning rust. That's the 2nd disclaimer, this is my rust learning project, so I will
probably have done silly things. 

## Running shred

```bash
cargo run
```

## Practice routine

I base my practice routine on [Justin Guitar's Intermediate Practice Routine](https://www.justinguitar.com/guitar-lessons/intermediate-practice-routine-pc-502). For now, that's not configurable. This routine takes 30 minutes consists of:
1. Technique
2. Repertoire
3. Transcribing
4. Knowledge
5. Improvisation 

### Technique

Finger Gym, Spider Exercise, Scale Picking

### Improvisation (10 mins)

Pick a key, practice. Initially this will just be major/minor pentatonic. 

### Repertoire

Keep track of the songs you want to play. Regularly practice old ones. Practice intros. Solos. Keep track of the solos
you want to learn. Stuff like that.

### Transcribing

Pick a song, figure it out. Can link to repertoire to find melodies to figure out.

### Knowledge

Music theory, where to find notes on the neck, ...

## Spaced repetition

The knowledge part, and certain exercises that I currently want to do, will make use of spaced repetition. Things like:
- What's the relative minor of C?
- What notes are in a C chord?

## Future

- Configure the length of my practice routine and adjust length of the parts
- Configure exercises in practice routine
- Configure exercises in each category
- Stats about practice
- Automatically generate theory questions:
  - relative minor of all notes
  - modes for a scale 
  - ...

