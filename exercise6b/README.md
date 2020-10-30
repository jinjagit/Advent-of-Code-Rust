# Part Two
  
Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).  
  
You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.  
  
For example, suppose you have the following map:  
```
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
```
Visually, the above map of orbits looks like this:  
```
                          YOU
                         /
        G - H       J - K - L
       /           /
COM - B - C - D - E - F
               \
                I - SAN
```
In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:  
```
    K to J
    J to E
    E to D
    D to I
```
Afterward, the map of orbits looks like this:  
```
        G - H       J - K - L
       /           /
COM - B - C - D - E - F
               \
                I - SAN
                 \
                  YOU
```
What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)  

# My notes:
  
In these graphs, a parent may have many children, but _a child can only have one parent_.  
  
Thus, we can simply start at each of the 2 given locations, and for each location, step up the chain of parents to COM, recording each step.  
  
Then, we compare the 2 lists of steps to identify the elements that are shared by both lists. Identify which element is the shared element that links the two points in the shortest path (use orbits count?)  
  
Then we add together the steps from the shared elment we found in the previous step to each of the 2 locations. (Again, this can probably be computed using the orbits counts).  
