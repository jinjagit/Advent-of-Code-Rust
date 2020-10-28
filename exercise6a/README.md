# Day 6: Universal Orbit Map
  
You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).  
  
Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:  
```
                  \
                   \
                    |
                    |
AAA--> o            o <--BBB
                    |
                    |
                   /
                  /
```
In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".  
  
Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.  
  
Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.  
  
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
```
```
Visually, the above map of orbits looks like this:

        G - H       J - K - L
       /           /
COM - B - C - D - E - F
               \
                I
```
In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.  
  
Here, we can count the total number of orbits as follows:  
  
    D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.  
    L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.  
    COM orbits nothing.  
  
The total number of direct and indirect orbits in this example is 42.  
  
What is the total number of direct and indirect orbits in your map data?  

## My thoughts:
  
This looks like a graph thing. But, at first glance (not reading through exercise thoroughly, and not researching how to implement a graph nor std or non-std libraries for graphs), my hunch is the follwing would work:  
  
Use a vec of structs. Read in each relationship from data as a struct:  
```
Struct SomeName {
  body: &str,
  orbiting: &str,
}
```
Note: could just use an array (not struct), but disadvantage is does not use named vars (? use hashmap with k/v pairs)

Split input string into vec of arrays, each array contains 2 strings [body, orbiting]  
  
Convert to a vec of structs.    
   
Then, pseudocode:   
```
let mut count = bodies.iter().count();
let total_orbits = 0;
let mut orbits = 1;
let mut parents vec = vec![]
let mut children vec = vec![]

for each body in bodies {
  if body.orbiting == "COM"
    body.orbits = orbits
    move body to parents vec (OR add / remove)
    total_orbits += orbits
    count -= 1
}

loop while count > 0 {
  orbits += 1

  for each body in parent {
    for each body in bodies {
      if bodies.body.orbiting == parents.body.name
      set bodies.body.orbits to parents.body.orbits + 1
      move bodies.body to children vec (OR add / remove)
      total_orbits += orbits
      count -= 1
    }
  }

  parents vec = children vec
  children vec = vec![]  
}
```
