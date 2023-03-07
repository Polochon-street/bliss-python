Python bindings for [bliss-rs](https://github.com/Polochon-street/bliss/tree/master/bliss-rs).

Audio library used as a building block to make playlists from songs.

Installation
============

bliss-audio is available for Python 3.5+ via pip:

```
$ pip install bliss-audio
```

Usage
=====

```python
from bliss_audio import Song
import numpy as np

song1 = Song("/path/to/song1")
song2 = Song("/path/to/song2")

distance = np.linalg.norm(np.array(song1.analysis) - np.array(song2.analysis))

print('Distance between song1 and song2 is {}'.format(distance))
```

Then you most likely want to analyze a bunch of songs like that, store the
result somewhere, and generate playlists on the fly by taking a song and
finding the next one by computing the one with the smallest distance to it.
