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
  
This looks like a graph thing. But, at first glance (not reading through exercise thoroughly, and not researching how to implement a graph nor std or non-std libraries for graphs), my hunch is this would work.  
  
Use a vec of structs. Read in each relationship from data as a struct:  
```
Struct SomeName {
  body: String,
  orbits: i32,
  orbiting: String,
}
```
Split input string into vec of arrays, each array contains 2 strings [body, orbiting]  
  
Initial body = Something, which has orbits = 1 and orbiting = 'COM'  
  
Could do math as add structs to vec (or could after).
Pseudocode:    
```
let mut total_orbits: i32 = 0;
let mut index = 0;
let previous_body: String = "nothing"

for array in vec {
  orbits = 0;
  
  if array[index].orbiting = COM {
    orbits = 1;
  } else if array[index].orbting = array[index -1].body {
    orbits = array[index - 1].orbits + 1;
  } else {
    orbits = find_struct(struct_with_orbiting_as_body).orbits + 1;
  }

  create struct using new info;
  add struct to vec of structs

  total_orbits += orbits

  index += 1;

}
```
1st tricky thing is to work out how to add structs to a vec (? need different names for each struct or not). DONE  
  
Let's read through instructions carefully, before going down this route! Confirmed - my basic assumptions are correct.  

Next steps:
1. Split input string into vec of component parts.
