# Dataset

Download a **Distance graph** (`.gr.gz`) and the matching **Coordinates** (`.co.gz`) from the [9th DIMACS Implementation Challenge](https://www.diag.uniroma1.it/challenge9/download.shtml). Both files must be the same network (for example `USA-road-d.NY`).

The program reads plain text, so decompress the archives into this folder:

```bash
gunzip USA-road-d.NY.gr.gz USA-road-d.NY.co.gz
```

That leaves `USA-road-d.NY.gr` (arcs) and `USA-road-d.NY.co` (coordinates). `src/main.rs` is hardcoded to `dataset/USA-road-d.USA.gr` and `dataset/USA-road-d.USA.co`; change those two paths to use another pair.
