# TODO list

## Implement polygon clipping algorithm (Greiner-Hormann)

It may be useful also for rectangle clipping. Current rectangle clipping algorithm (part of the shader)
is not able to work correctly with rotations.

Links:
- http://davis.wpi.edu/~matt/courses/clipping/
- https://gitlab.com/nathanfaucett/rs-polygon2/blob/master/src/clipping/greiner_hormann/polygon.rs
- Max K. Agoston "Computer Graphics and Geometric Modelling" (2004)

## Upgrade clipping algorithm to operate on Bezier curves
