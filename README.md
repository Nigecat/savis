This tool provides a framework to generate state animations of [Simulated Annealing](https://en.wikipedia.org/wiki/Simulated_annealing) algorithm implementations. Though since this was originally intended just for my use, it isn't all that user friendly. 

# Prerequisites 

 - [Python](https://www.python.org/)
 - [Rust](https://rustup.rs/) (+`cargo`)
 - The `json_stream` pip package (can be installed via `pip install json_stream`)

# Usage

The general flow is to use the [python library](lib) to generate the data, then give it to the rust code to create the visualization.  
To do this, you will need to import [lib/__init__.py](lib/__init__.py) into the python code that contains the algorithm implementation.  
You can do this by either copying/renaming the file into a local directory, copying the contents directly into your file, pip installing it from source, etc...  
If you want a particularly hacky way to do it, you can use the following code to import it temporarily:  
```python
import sys
sys.path.append("/path/to/this/repo/.../savis")
import lib as savis  # type: ignore
```
Once you have the import available as `savis`, you can continue on.

First, you must initialize an instance of `savis.Chronicle`, this takes:
 - a list of graph nodes (each one must be unique)
 - a list of graph edges (as tuples of graph nodes, `(a, b)` and `(b, a)` are considered equivelent so only one should be given)
 - a list of node positions, depending on your graphing library you may be able to dump this out directly (as a dictionary of `{ $node: (x, y) }` where `(0, 0)` is the center of the screen and the corners are `(+-1, +-1)`

 - 
