# Day 10: Monitoring Station

You fly into the asteroid belt and reach the Ceres monitoring station. The Elves here have an emergency: they're having trouble tracking all of the asteroids and can't be sure they're safe.

The Elves would like to build a new monitoring station in a nearby area of space; they hand you a map of all of the asteroids in that region (your puzzle input).

The map indicates whether each position is empty (.) or contains an asteroid (#). The asteroids are much smaller than they appear on the map, and every asteroid is exactly in the center of its marked position. The asteroids can be described with X,Y coordinates where X is the distance from the left edge and Y is the distance from the top edge (so the top-left corner is 0,0 and the position immediately to its right is 1,0).

Your job is to figure out which asteroid would be the best place to build a new monitoring station. A monitoring station can detect any asteroid to which it has direct line of sight - that is, there cannot be another asteroid exactly between them. This line of sight can be at any angle, not just lines aligned to the grid or diagonally. The best location is the asteroid that can detect the largest number of other asteroids.

For example, consider the following map:
```
.#..#
.....
#####
....#
...##
```
The best location for a new monitoring station on this map is the highlighted asteroid at 3,4 because it can detect 8 asteroids, more than any other location. (The only asteroid it cannot detect is the one at 1,0; its view of this asteroid is blocked by the asteroid at 2,2.) All other asteroids are worse locations; they can detect 7 or fewer other asteroids. Here is the number of other asteroids a monitoring station on each asteroid could detect:
```
.7..7
.....
67775
....7
...87
```
Here is an asteroid (#) and some examples of the ways its line of sight might be blocked. If there were another asteroid at the location of a capital letter, the locations marked with the corresponding lowercase letter would be blocked and could not be detected:
```
#.........
...A......
...B..a...
.EDCG....a
..F.c.b...
.....c....
..efd.c.gb
.......c..
....f...c.
...e..d..c
```
Here are some larger examples:

    Best is 5,8 with 33 other asteroids detected:

    ......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####

    Best is 1,2 with 35 other asteroids detected:

    #.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.

    Best is 6,3 with 41 other asteroids detected:

    .#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..

    Best is 11,13 with 210 other asteroids detected:

    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##

# My notes:

This seems very similar to how I worked out which pieces were pinned in my chess app.  
  
Basic method is probably:  

Get count of all asteroids. 'Visible' asteroids from an asteroid = count - obscured asteroids - 1  
```
Loop through each asteroid, considering each one as a potential 'base' site
  For each asteroid (that is not the one being consider as the base), calculate vector (as (dx, dy)) from the considered 'base'.  
  Reduce each vector to minimum sized integer pair possible (using gcd) = direction. Store each direction vector with respective asteroid coordinates (structs).  
  Then loop through a list of all directions created:  
    Move 1st to (initially empty) 'same_vector_list' + any others with same vector.  
      If 'same_direction_list' count == 1, then do nothing    
      If 'same_direction_list' count > 1, then add count - 1 to 'obscured_count'   
  End loop   
  Store n of visible asteroids, and coords of asteroid considerd for 'base', if better than 'best' so far    
End loop    
Return asteroid coords with highest n of visible asteroids 
```
The above should work, but is not optimized. Possible optimizations include:    
  Storing relations between asteroids, since this data could be re-used to discover 'obscured' asteroids when considering a different asteroid 'base'.  
  All asteroids on neighboring rows or columns will be visible.  
Whilst these optimizations would reduce time taken (and ops) to find 'best' asteroid, I am not sure it would make a significant enough difference to justify the increased complexity (could be wrong).      