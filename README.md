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

---

First, you must initialize an instance of `savis.Chronicle`, this takes:
 - a list of graph nodes (each one must be unique)
 - a list of graph edges (as tuples of graph nodes, `(a, b)` and `(b, a)` are considered equivelent so only one should be given)
 - a list of node positions, depending on your graphing library you may be able to dump this out directly (as a dictionary of `{ $node: (x, y) }` where `(0, 0)` is the center of the screen and the corners are `(+-1, +-1)`

For instance, if you had three nodes: A, B, C, D and wanted to display them in a square with an edge A -> B -> C -> D, then you could do the following:
```python
S = savis.Chronicle(
    ["A", "B", "C", "D"],
    [
        ("A", "B"),
        ("B", "C"),
        ("C", "D")
    ],
    {
        "A": (-0.5, 0.5),
        "B": (0.5, 0.5),
        "C": (-0.5, -0.5),
        "D": (0.5, -0.5)
    }
)
```

---

Then, you must call `S.emit(..)` in each of your iterations. This function takes an instance of `savis.State` with the following parameters:
 - `temperature` - the current system temperature
 - `energy` - the current system energy
 - `used` - whether this state was used (True if yes, False if it was discarded)
 - `route` - the route, as an array of `savis.Node(NAME)` instances where `NAME` is the node string you passed to the `savis.Chronicle` constructor

So for instance if our state had a temperature of 20, an energy value of 10, a route of A -> B -> C, and it was not discarded, then we would call:
```python
S.emit(savis.State(temperature=20.0, energy=10.0, used=True, route=[savis.Node("A"), savis.Node("B"), savis.Node("C")]))
```
(these values should be determined dynamically from the application state in actual usage)

---

Once done iterating, call `S.save("FILE.json")` to save the contents out to a file. 
Putting it all together, the following example represents two states, where the route changes from A -> B to A -> B -> C after lowing both the temperature and energy:
```python
import lib as savis

S = savis.Chronicle(["A", "B", "C", "D"], [("A", "B"), ("B", "C"), ("C", "D")], { "A": (-0.5, 0.5), "B": (0.5, 0.5), "C": (-0.5, -0.5), "D": (0.5, -0.5) })

S.emit(savis.State(temperature=25.0, energy=12.0, used=True, route=[savis.Node("A"), savis.Node("B"), savis.Node("C")]))
S.emit(savis.State(temperature=20.0, energy=10.0, used=True, route=[savis.Node("A"), savis.Node("B"), savis.Node("C")]))

S.save("example.json")
```

---

Then, you just need to simply pass the file to the Rust program. Assuming you have `cargo` installed and are `cd`-d into this repository, run `cargo run --release -- /path/to/FILE.json --delay T` where `FILE.json` is the output file from the previous step and `T` is the number of miliseconds between updates (ie how long to wait before showing the state of the next iteration). Assuming you did this correctly, the above example should yield the following final state:
![image](https://github.com/Nigecat/savis/assets/48661288/f09d210b-2a81-4c1c-9975-75a0389e7496)


