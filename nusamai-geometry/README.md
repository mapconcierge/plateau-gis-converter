# nusamai-geometry

A compact, zero-copy geometry representation.

This library avoids using jagged arrays (i.e. Vector of Vector of ...) to represent MultiPolygon/Polygon/etc.

## Visual examples of the data structure

### LineString

![LineString](./docs/01_linestring.png)

### Polygon

![Polygon](./docs/02_polygon.png)

### Polygon with a hole

![Polygon with a hole](./docs/03_polygon_with_a_hole.png)

### Polygon with multiple holes

![Polygon with multiple holes](./docs/04_polygon_with_multiple_holes.png)

### MultiPolygon

![MultiPolygon](./docs/05_multipolygon.png)

### MultiPolygon with holes

![MultiPolygon with holes](./docs/06_multipolygon_with_holes.png)

### Multiple polygons, multiple holes

![Multiple polygons, multiple holes](./docs/07_multipolygon_multiple_holes.png)